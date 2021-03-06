// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use core::error_messages::{EvelynBaseError, EvelynCoreError, EvelynServiceError};
use core::user;
use model;
use processing;
use serde_json;
use server::routing::{RouterInput, RouterOutput};
use std::sync::Arc;

pub fn create_user_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    let request_model_decoded: Result<model::user::CreateUserRequestModel, _> = serde_json::from_str(&router_input.request_body);

    match request_model_decoded {
        Ok(request_model) => {
            match user::create_user(request_model, processor_data) {
                None => {
                    RouterOutput {
                        response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                                                                 error: None,
                                                             })
                                .unwrap(),
                    }
                },
                Some(e) => {
                    match e {
                        EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists(EvelynBaseError::NothingElse) => {
                            RouterOutput {
                                response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                                                                         error: Some(From::from(EvelynServiceError::UserAlreadyExists(e))),
                                                                     })
                                        .unwrap(),
                            }
                        },
                        _ => {
                            RouterOutput {
                                response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                                                                         error: Some(From::from(EvelynServiceError::CreateUser(e))),
                                                                     })
                                        .unwrap(),
                            }
                        },
                    }
                },
            }
        },
        Err(e) => {
            let model: model::ErrorModel = From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e));
            RouterOutput {
                response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                                                         error: Some(model),
                                                     })
                        .unwrap(),
            }
        },
    }
}

pub fn logon_user_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    let request_model_de: Result<model::user::LogonUserRequestModel, _> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
        Ok(request_model) => {
            match user::logon_user(request_model, processor_data) {
                Ok(response) => {
                    RouterOutput {
                        response_body: serde_json::to_string(&response).unwrap(),
                    }
                },
                Err(e) => {
                    match e {
                        EvelynCoreError::InvalidLogon(EvelynBaseError::NothingElse) => {
                            RouterOutput {
                                response_body: serde_json::to_string(&model::user::LogonUserResponseModel {
                                                                         token: None,
                                                                         error: Some(From::from(EvelynServiceError::LogonUser(e))),
                                                                     })
                                        .unwrap(),
                            }
                        },
                        _ => {
                            RouterOutput {
                                response_body: serde_json::to_string(&model::user::LogonUserResponseModel {
                                                                         token: None,
                                                                         error: Some(From::from(EvelynServiceError::FailedToLogonUser(e))),
                                                                     })
                                        .unwrap(),
                            }
                        },
                    }
                },
            }
        },
        Err(e) => {
            RouterOutput {
                response_body: serde_json::to_string(&model::user::LogonUserResponseModel {
                                                         token: None,
                                                         error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e))),
                                                     })
                        .unwrap(),
            }
        },
    }
}

pub fn search_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(model::user::SearchRequestModel, router_input) {
        Ok(request_model) => {
            validate_session!(processor_data, request_model);

            match user::search_for_users(request_model, processor_data) {
                Ok(response) => {
                    model_to_router_output!(response)
                },
                Err(e) => {
                    model_to_router_output!(model::user::SearchResponseModel {
                        search_results: Vec::new(),
                        error: service_error_to_model!(EvelynServiceError::SearchForUsers(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(model::user::SearchResponseModel {
                search_results: Vec::new(),
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}
