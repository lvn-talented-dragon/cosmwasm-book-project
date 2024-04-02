use crate::msg::{GreetResp, InstantiateMsg, QueryMsg};
use crate::state::ADMINS;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
        AdminsList {} => to_binary(&query::admins_list(deps)?),
    }
}

#[allow(dead_code)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

mod query {
    use crate::msg::AdminListResp;

    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }

    pub fn admins_list(deps: Deps) -> StdResult<AdminListResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp = AdminListResp { admins };
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use std::{any::Any, vec};

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
    };
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::msg::AdminListResp;

    use super::*;

    #[test]
    fn greet_query_module() {
        let resp = query::greet().unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }

    #[test]
    fn greet_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            InstantiateMsg { admins: vec![] },
        )
        .unwrap();

        let resp = query(mock_dependencies().as_ref(), mock_env(), QueryMsg::Greet {}).unwrap();
        let resp: GreetResp = from_binary(&resp).unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }

    #[test]
    fn greet_query_multitest() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![] },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }

    #[test]
    fn admins_list_query_multitest() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![] },
                &[],
                "Contract 1",
                None,
            )
            .unwrap();

        let resp: AdminListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(resp, AdminListResp { admins: vec![] });

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                },
                &[],
                "Contract 2",
                None,
            )
            .unwrap();

        let resp: AdminListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(
            resp,
            AdminListResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")]
            }
        );
    }
}
