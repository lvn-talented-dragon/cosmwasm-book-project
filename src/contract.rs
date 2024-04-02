use crate::msg::{GreetResp, QueryMsg};
use cosmwasm_std::{
  to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};

pub fn instantiate(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: Empty
) -> StdResult<Response> {
  Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  use QueryMsg::*;

  match msg {
    Greet {} => to_binary(&query::greet()?)
  }
}

#[allow(dead_code)]
pub fn execute(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: Empty
) -> StdResult<Response> {
  unimplemented!()
}

mod query {
  use super::*;

  pub fn greet() -> StdResult<GreetResp> {
    let resp = GreetResp {
      message: "Hello World".to_owned()
    };

    Ok(resp)
  }
}

#[cfg(test)]
mod test {
  use std::any::Any;

use cosmwasm_std::{from_binary, Addr, testing::{mock_dependencies, mock_env, mock_info}};
  use cw_multi_test::{App, ContractWrapper, Executor};

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
      Empty {},
    )
    .unwrap();

    let resp = query(
      mock_dependencies().as_ref(),
      mock_env(),
      QueryMsg::Greet {}
    ).unwrap();
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
        &Empty {},
        &[],
        "Contract",
        None
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
}