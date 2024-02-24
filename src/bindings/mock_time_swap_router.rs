pub use mock_time_swap_router::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mock_time_swap_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_WETH9"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WETH9"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH9"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exactInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exactInput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISwapRouter.ExactInputParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exactInputSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exactInputSingle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISwapRouter.ExactInputSingleParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exactOutput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exactOutput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISwapRouter.ExactOutputParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exactOutputSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exactOutputSingle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISwapRouter.ExactOutputSingleParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refundETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("refundETH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("selfPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("selfPermitAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitAllowedIfNecessary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "selfPermitAllowedIfNecessary",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitIfNecessary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "selfPermitIfNecessary",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTime"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_time"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepTokenWithFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepTokenWithFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeBips"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeRecipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrapWETH9"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapWETH9"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrapWETH9WithFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapWETH9WithFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeBips"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeRecipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKTIMESWAPROUTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R`\0\x19`\0U4\x80\x15b\0\0\x17W`\0\x80\xFD[P`@Qb\0'\xEB8\x03\x80b\0'\xEB\x839\x81\x01`@\x81\x90Rb\0\0:\x91b\0\0vV[`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x91\x1B\x16`\xA0Rb\0\0\xADV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0qW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\x89W\x81\x82\xFD[b\0\0\x94\x83b\0\0YV[\x91Pb\0\0\xA4` \x84\x01b\0\0YV[\x90P\x92P\x92\x90PV[`\x80Q``\x1C`\xA0Q``\x1Ca&\xE2b\0\x01\t`\09\x80a\x01\x1DR\x80a\x04\xF7R\x80a\x05\xE2R\x80a\x06oR\x80a\x06\xAFR\x80a\x07\x9AR\x80a\x17SR\x80a\x17\x99R\x80a\x18\rRP\x80a\x0B\xD3R\x80a\x10\xCER\x80a\x18\xE8RPa&\xE2`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\rW`\x005`\xE0\x1C\x80c\xC0K\x8DY\x11a\0\x95W\x80c\xDF*\xB5\xBB\x11a\0dW\x80c\xDF*\xB5\xBB\x14a\x02\xAFW\x80c\xE0\xE1\x89\xA0\x14a\x02\xC2W\x80c\xF2\x8C\x04\x98\x14a\x02\xD5W\x80c\xF3\x99\\g\x14a\x02\xE8W\x80c\xFAF\x1E3\x14a\x02\xFBWa\x01}V[\x80c\xC0K\x8DY\x14a\x02aW\x80c\xC2\xE3\x14\n\x14a\x02tW\x80c\xC4Z\x01U\x14a\x02\x87W\x80c\xDB>!\x98\x14a\x02\x9CWa\x01}V[\x80cI@K|\x11a\0\xDCW\x80cI@K|\x14a\x01\xE6W\x80cJ\xA4\xA4\xFC\x14a\x01\xF9W\x80c\x9B,\n7\x14a\x02\x1BW\x80c\xA4\xA7\x8F\x0C\x14a\x02.W\x80c\xAC\x96P\xD8\x14a\x02AWa\x01}V[\x80c\x12!\x0E\x8A\x14a\x01\x82W\x80c;\xEB&\xC4\x14a\x01\x8AW\x80cAK\xF3\x89\x14a\x01\xAAW\x80cFY\xA4\x94\x14a\x01\xD3Wa\x01}V[6a\x01}W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01{W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhNot WETH9`\xB8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\0[`\0\x80\xFD[a\x01{a\x03\x1BV[4\x80\x15a\x01\x96W`\0\x80\xFD[Pa\x01{a\x01\xA56`\x04a#cV[a\x03-V[a\x01\xBDa\x01\xB86`\x04a\"\\V[a\x032V[`@Qa\x01\xCA\x91\x90a%\xCDV[`@Q\x80\x91\x03\x90\xF3[a\x01{a\x01\xE16`\x04a\x1F\xDAV[a\x04YV[a\x01{a\x01\xF46`\x04a#{V[a\x04\xF3V[4\x80\x15a\x02\x05W`\0\x80\xFD[Pa\x02\x0Ea\x06mV[`@Qa\x01\xCA\x91\x90a$jV[a\x01{a\x02)6`\x04a#\xAAV[a\x06\x91V[a\x01{a\x02<6`\x04a\x1F\xDAV[a\x08]V[a\x02Ta\x02O6`\x04a :V[a\x08\xEEV[`@Qa\x01\xCA\x91\x90a$\xC4V[a\x01\xBDa\x02o6`\x04a!\xB1V[a\n.V[a\x01{a\x02\x826`\x04a\x1F\xDAV[a\x0BBV[4\x80\x15a\x02\x93W`\0\x80\xFD[Pa\x02\x0Ea\x0B\xD1V[a\x01\xBDa\x02\xAA6`\x04a\"\\V[a\x0B\xF5V[a\x01{a\x02\xBD6`\x04a\x1F;V[a\r\x1CV[a\x01{a\x02\xD06`\x04a\x1F|V[a\r\xFAV[a\x01\xBDa\x02\xE36`\x04a\"xV[a\x0F!V[a\x01{a\x02\xF66`\x04a\x1F\xDAV[a\x10\x17V[4\x80\x15a\x03\x07W`\0\x80\xFD[Pa\x01{a\x03\x166`\x04a \xCCV[a\x10\x89V[G\x15a\x03+Wa\x03+3Ga\x11\x9CV[V[`\x01UV[`\0\x81`\x80\x015\x80a\x03Ba\x12\x8BV[\x11\x15a\x03\x8BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x04$`\xA0\x84\x015a\x03\xA3`\x80\x86\x01``\x87\x01a\x1F\x18V[a\x03\xB4a\x01\0\x87\x01`\xE0\x88\x01a\x1F\x18V[`@\x80Q\x80\x82\x01\x90\x91R\x80a\x03\xCC` \x8A\x01\x8Aa\x1F\x18V[a\x03\xDC``\x8B\x01`@\x8C\x01a#@V[a\x03\xEC`@\x8C\x01` \x8D\x01a\x1F\x18V[`@Q` \x01a\x03\xFE\x93\x92\x91\x90a$\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x12\x91V[\x91P\x82`\xC0\x015\x82\x10\x15a\x04SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%cV[`@Q\x80\x91\x03\x90\xFD[P\x91\x90PV[`@\x80Qc#\xF2\xEB\xC3`\xE2\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\x01`\x84\x82\x01R`\xFF\x85\x16`\xA4\x82\x01R`\xC4\x81\x01\x84\x90R`\xE4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\x8F\xCB\xAF\x0C\x91a\x01\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x04\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE7W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05bW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05vW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x05\x8CW`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x05\xDAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x06hW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06ZW=`\0\x80>=`\0\xFD[PPPPa\x06h\x82\x82a\x11\x9CV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x82\x11\x80\x15a\x06\xA2WP`d\x82\x11\x15[a\x06\xABW`\0\x80\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\x1AW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x07.W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x07DW`\0\x80\xFD[PQ\x90P\x84\x81\x10\x15a\x07\x92W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x08VW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x12W=`\0\x80>=`\0\xFD[PPPP`\0a'\x10a\x08.\x85\x84a\x13\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81a\x085W\xFE[\x04\x90P\x80\x15a\x08HWa\x08H\x83\x82a\x11\x9CV[a\x08T\x85\x82\x84\x03a\x11\x9CV[P[PPPPPV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q`\0\x19\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xAEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\xD8W`\0\x80\xFD[PQ\x10\x15a\x08TWa\x08T\x86\x86\x86\x86\x86\x86a\x04YV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\t\x07W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t;W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t&W\x90P[P\x90P`\0[\x82\x81\x10\x15a\n'W`\0\x800\x86\x86\x85\x81\x81\x10a\tYW\xFE[\x90P` \x02\x81\x01\x90a\tk\x91\x90a%\xD6V[`@Qa\ty\x92\x91\x90a$ZV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\t\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xB9V[``\x91P[P\x91P\x91P\x81a\n\x05W`D\x81Q\x10\x15a\t\xD2W`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\t\xEC\x91\x90a!GV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x91\x90a%$V[\x80\x84\x84\x81Q\x81\x10a\n\x12W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\tAV[P\x92\x91PPV[`\0\x81`@\x01Q\x80a\n>a\x12\x8BV[\x11\x15a\n\x87W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[3[`\0a\n\x98\x85`\0\x01Qa\x14\rV[\x90Pa\n\xE4\x85``\x01Q\x82a\n\xB1W\x86` \x01Qa\n\xB3V[0[`\0`@Q\x80`@\x01`@R\x80a\n\xCD\x8B`\0\x01Qa\x14\x19V[\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x12\x91V[``\x86\x01R\x80\x15a\x0B\x04W\x84Q0\x92Pa\n\xFD\x90a\x14(V[\x85Ra\x0B\x11V[\x84``\x01Q\x93PPa\x0B\x17V[Pa\n\x89V[\x83`\x80\x01Q\x83\x10\x15a\x0B;W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%cV[PP\x91\x90PV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q\x86\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0B\x91W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0B\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0B\xBBW`\0\x80\xFD[PQ\x10\x15a\x08TWa\x08T\x86\x86\x86\x86\x86\x86a\x10\x17V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x81`\x80\x015\x80a\x0C\x05a\x12\x8BV[\x11\x15a\x0CNW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0C\xEA`\xA0\x84\x015a\x0Cf`\x80\x86\x01``\x87\x01a\x1F\x18V[a\x0Cwa\x01\0\x87\x01`\xE0\x88\x01a\x1F\x18V[`@Q\x80`@\x01`@R\x80\x88` \x01` \x81\x01\x90a\x0C\x95\x91\x90a\x1F\x18V[a\x0C\xA5``\x8B\x01`@\x8C\x01a#@V[a\x0C\xB2` \x8C\x01\x8Ca\x1F\x18V[`@Q` \x01a\x0C\xC4\x93\x92\x91\x90a$\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x14?V[\x91P\x82`\xC0\x015\x82\x11\x15a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%7V[P`\0\x19`\0U\x91\x90PV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rkW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\r\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\r\x95W`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\r\xE3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\r\xF4Wa\r\xF4\x84\x83\x83a\x15\xBAV[PPPPV[`\0\x82\x11\x80\x15a\x0E\x0BWP`d\x82\x11\x15[a\x0E\x14W`\0\x80\xFD[`\0\x85`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0EcW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0EwW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E\x8DW`\0\x80\xFD[PQ\x90P\x84\x81\x10\x15a\x0E\xDBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x08TW`\0a'\x10a\x0E\xF0\x83\x86a\x13\xE3V[\x81a\x0E\xF7W\xFE[\x04\x90P\x80\x15a\x0F\x0BWa\x0F\x0B\x87\x84\x83a\x15\xBAV[a\x0F\x18\x87\x86\x83\x85\x03a\x15\xBAV[PPPPPPPV[`\0\x81`@\x015\x80a\x0F1a\x12\x8BV[\x11\x15a\x0FzW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0F\xED``\x84\x015a\x0F\x92`@\x86\x01` \x87\x01a\x1F\x18V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x90\x80a\x0F\xAA\x89\x80a%\xD6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP3` \x90\x91\x01Ra\x14?V[P`\0T\x91P\x82`\x80\x015\x82\x11\x15a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%7V[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\xFF\x85\x16`\x84\x82\x01R`\xA4\x81\x01\x84\x90R`\xC4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x04\xD3W`\0\x80\xFD[`\0\x84\x13\x80a\x10\x98WP`\0\x83\x13[a\x10\xA1W`\0\x80\xFD[`\0a\x10\xAF\x82\x84\x01\x84a\"\xB0V[\x90P`\0\x80`\0a\x10\xC3\x84`\0\x01Qa\x17\x01V[\x92P\x92P\x92Pa\x10\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\x172V[P`\0\x80`\0\x8A\x13a\x11\x1CW\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10\x89a\x113V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x8A[\x91P\x91P\x81\x15a\x11RWa\x11M\x85\x87` \x01Q3\x84a\x17QV[a\x04\xE7V[\x85Qa\x11]\x90a\x14\rV[\x15a\x11\x82W\x85Qa\x11m\x90a\x14(V[\x86Ra\x11|\x813`\0\x89a\x14?V[Pa\x04\xE7V[\x80`\0\x81\x90UP\x83\x94Pa\x04\xE7\x85\x87` \x01Q3\x84a\x17QV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x11\xE8W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x11\xC9V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12JW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12OV[``\x91P[PP\x90P\x80a\x06hW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01T\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16a\x12\xA5W0\x93P[`\0\x80`\0a\x12\xB7\x85`\0\x01Qa\x17\x01V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x90\x84\x16\x10`\0\x80a\x12\xDB\x86\x86\x86a\x18\xE1V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x8B\x85a\x12\xF4\x8Fa\x19\x1FV[`\x01`\x01`\xA0\x1B\x03\x8E\x16\x15a\x13\tW\x8Da\x13/V[\x87a\x13(Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x13/V[d\x01\0\x02v\xA4[\x8D`@Q` \x01a\x13@\x91\x90a%\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13o\x95\x94\x93\x92\x91\x90a$~V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xC0\x91\x90a \xA9V[\x91P\x91P\x82a\x13\xCFW\x81a\x13\xD1V[\x80[`\0\x03\x9B\x9APPPPPPPPPPPV[`\0\x82\x15\x80a\x13\xFEWPP\x81\x81\x02\x81\x83\x82\x81a\x13\xFBW\xFE[\x04\x14[a\x14\x07W`\0\x80\xFD[\x92\x91PPV[\x80Q`B\x11\x15[\x91\x90PV[``a\x14\x07\x82`\0`+a\x195V[\x80Q``\x90a\x14\x07\x90\x83\x90`\x17\x90`\x16\x19\x01a\x195V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16a\x14SW0\x93P[`\0\x80`\0a\x14e\x85`\0\x01Qa\x17\x01V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10`\0\x80a\x14\x89\x85\x87\x86a\x18\xE1V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x8B\x85a\x14\xA2\x8Fa\x19\x1FV[`\0\x03`\x01`\x01`\xA0\x1B\x03\x8E\x16\x15a\x14\xBAW\x8Da\x14\xE0V[\x87a\x14\xD9Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x14\xE0V[d\x01\0\x02v\xA4[\x8D`@Q` \x01a\x14\xF1\x91\x90a%\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15 \x95\x94\x93\x92\x91\x90a$~V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x159W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15q\x91\x90a \xA9V[\x91P\x91P`\0\x83a\x15\x86W\x81\x83`\0\x03a\x15\x8CV[\x82\x82`\0\x03[\x90\x98P\x90P`\x01`\x01`\xA0\x1B\x03\x8A\x16a\x15\xABW\x8B\x81\x14a\x15\xABW`\0\x80\xFD[PPPPPPP\x94\x93PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x166W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x16\x17V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x16\x98W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x16\x9DV[``\x91P[P\x91P\x91P\x81\x80\x15a\x16\xCBWP\x80Q\x15\x80a\x16\xCBWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x16\xC8W`\0\x80\xFD[PQ[a\x08VW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x80a\x17\x0F\x84\x82a\x1A\x86V[\x92Pa\x17\x1C\x84`\x14a\x1B6V[\x90Pa\x17)\x84`\x17a\x1A\x86V[\x91P\x91\x93\x90\x92PV[`\0a\x17H\x85a\x17C\x86\x86\x86a\x1B\xDDV[a\x1C3V[\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\x17\x92WP\x80G\x10\x15[\x15a\x18\xB4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x17\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x06W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18\xACW`\0\x80\xFD[Pa\r\xF4\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14\x15a\x18\xD5Wa\x18\xD0\x84\x83\x83a\x15\xBAV[a\r\xF4V[a\r\xF4\x84\x84\x84\x84a\x1CVV[`\0a\x19\x17\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x12\x86\x86\x86a\x1B\xDDV[a\x1D\xA6V[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\x191W`\0\x80\xFD[P\x90V[``\x81\x82`\x1F\x01\x10\x15a\x19\x80W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82\x82\x84\x01\x10\x15a\x19\xC8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81\x83\x01\x84Q\x10\x15a\x1A\x14W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[``\x82\x15\x80\x15a\x1A3W`@Q\x91P`\0\x82R` \x82\x01`@Ra\x1A}V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x1AlW\x80Q\x83R` \x92\x83\x01\x92\x01a\x1ATV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[`\0\x81\x82`\x14\x01\x10\x15a\x1A\xD5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x14\x01\x83Q\x10\x15a\x1B&W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81\x82`\x03\x01\x10\x15a\x1B\x84W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x03\x01\x83Q\x10\x15a\x1B\xD4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01`\x03\x01Q\x90V[a\x1B\xE5a\x1E\x8AV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x1C\x03W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0a\x1C?\x83\x83a\x1D\xA6V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x14\x07W`\0\x80\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x1C\xDAW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x1C\xBBV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1D<W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1DAV[``\x91P[P\x91P\x91P\x81\x80\x15a\x1DoWP\x80Q\x15\x80a\x1DoWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x1DlW`\0\x80\xFD[PQ[a\x08TW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb)\xAA#`\xE9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x1D\xCEW`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[\x805a\x14\x14\x81a&\x94V[`\0\x82`\x1F\x83\x01\x12a\x1E\xC5W\x80\x81\xFD[\x815a\x1E\xD8a\x1E\xD3\x82a&FV[a&\"V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E\xECW\x82\x83\xFD[\x81` \x85\x01` \x83\x017\x90\x81\x01` \x01\x91\x90\x91R\x92\x91PPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x04SW\x80\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x1F)W\x80\x81\xFD[\x815a\x1F4\x81a&\x94V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1FOW\x81\x82\xFD[\x835a\x1FZ\x81a&\x94V[\x92P` \x84\x015\x91P`@\x84\x015a\x1Fq\x81a&\x94V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1F\x93W\x80\x81\xFD[\x855a\x1F\x9E\x81a&\x94V[\x94P` \x86\x015\x93P`@\x86\x015a\x1F\xB5\x81a&\x94V[\x92P``\x86\x015\x91P`\x80\x86\x015a\x1F\xCC\x81a&\x94V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xF2W\x80\x81\xFD[\x865a\x1F\xFD\x81a&\x94V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a  W\x81\x82\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a LW\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a cW\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a vW\x83\x84\xFD[\x815\x81\x81\x11\x15a \x84W\x84\x85\xFD[\x86` \x80\x83\x02\x85\x01\x01\x11\x15a \x97W\x84\x85\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a \xBBW\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a \xE1W\x81\x82\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\x06W\x83\x84\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a!\x19W\x83\x84\xFD[\x815\x81\x81\x11\x15a!'W\x84\x85\xFD[\x88` \x82\x85\x01\x01\x11\x15a!8W\x84\x85\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a!XW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!nW\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13a!~W\x81\x82\xFD[\x80Qa!\x8Ca\x1E\xD3\x82a&FV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a!\xA0W\x83\x84\xFD[a\x17H\x82` \x83\x01` \x86\x01a&hV[`\0` \x82\x84\x03\x12\x15a!\xC2W\x80\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xD9W\x82\x83\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a!\xECW\x82\x83\xFD[`@Q`\xA0\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\"\x01W\xFE[`@R\x825\x82\x81\x11\x15a\"\x12W\x84\x85\xFD[a\"\x1E\x87\x82\x86\x01a\x1E\xB5V[\x82RPa\"-` \x84\x01a\x1E\xAAV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0a\x01\0\x82\x84\x03\x12\x15a\"nW\x80\x81\xFD[a\x1F4\x83\x83a\x1F\x06V[`\0` \x82\x84\x03\x12\x15a\"\x89W\x80\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x9FW\x81\x82\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x1F4W\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15a\"\xC1W\x80\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD8W\x82\x83\xFD[\x90\x83\x01\x90`@\x82\x86\x03\x12\x15a\"\xEBW\x82\x83\xFD[`@Q`@\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a#\0W\xFE[`@R\x825\x82\x81\x11\x15a#\x11W\x84\x85\xFD[a#\x1D\x87\x82\x86\x01a\x1E\xB5V[\x82RP` \x83\x015\x92Pa#0\x83a&\x94V[` \x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a#QW\x80\x81\xFD[\x815b\xFF\xFF\xFF\x81\x16\x81\x14a\x1F4W\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15a#tW\x80\x81\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\x8DW\x81\x82\xFD[\x825\x91P` \x83\x015a#\x9F\x81a&\x94V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a#\xBFW\x81\x82\xFD[\x845\x93P` \x85\x015a#\xD1\x81a&\x94V[\x92P`@\x85\x015\x91P``\x85\x015a#\xE8\x81a&\x94V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x81Q\x80\x84Ra$\x0B\x81` \x86\x01` \x86\x01a&hV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\0\x82\x84\x837\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a$\xB9\x90\x83\x01\x84a#\xF3V[\x97\x96PPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x84\x82\x02\x87\x01\x01\x92P\x83\x87\x01\x85[\x82\x81\x10\x15a%\x17W`?\x19\x88\x86\x03\x01\x84Ra%\x05\x85\x83Qa#\xF3V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a$\xE9V[P\x92\x97\x96PPPPPPPV[`\0` \x82Ra\x1F4` \x83\x01\x84a#\xF3V[` \x80\x82R`\x12\x90\x82\x01Rq\x15\x1B\xDB\xC8\x1B]X\xDA\x08\x1C\x99\\]Y\\\xDD\x19Y`r\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x13\x90\x82\x01Rr\x15\x1B\xDB\xC8\x1B\x1A]\x1D\x1B\x19H\x1C\x99X\xD9Z]\x99Y`j\x1B`@\x82\x01R``\x01\x90V[`\0` \x82R\x82Q`@` \x84\x01Ra%\xAC``\x84\x01\x82a#\xF3V[` \x94\x90\x94\x01Q`\x01`\x01`\xA0\x1B\x03\x16`@\x93\x90\x93\x01\x92\x90\x92RP\x90\x91\x90PV[\x90\x81R` \x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a%\xECW\x82\x83\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\x06W\x82\x83\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a&\x1BW`\0\x80\xFD[\x92P\x92\x90PV[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&>W\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&ZW\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a&\x83W\x81\x81\x01Q\x83\x82\x01R` \x01a&kV[\x83\x81\x11\x15a\r\xF4WPP`\0\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a&\xA9W`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 \x16\xD8\x9D\xDA7'R,\x15\xE0\x97\x88\xCF\xB0\xE1\xD9\x07\xEEc\xFEv\xAC-\xE0\x11\xC0Ov\xBB\xC9\xE2(dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static MOCKTIMESWAPROUTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\rW`\x005`\xE0\x1C\x80c\xC0K\x8DY\x11a\0\x95W\x80c\xDF*\xB5\xBB\x11a\0dW\x80c\xDF*\xB5\xBB\x14a\x02\xAFW\x80c\xE0\xE1\x89\xA0\x14a\x02\xC2W\x80c\xF2\x8C\x04\x98\x14a\x02\xD5W\x80c\xF3\x99\\g\x14a\x02\xE8W\x80c\xFAF\x1E3\x14a\x02\xFBWa\x01}V[\x80c\xC0K\x8DY\x14a\x02aW\x80c\xC2\xE3\x14\n\x14a\x02tW\x80c\xC4Z\x01U\x14a\x02\x87W\x80c\xDB>!\x98\x14a\x02\x9CWa\x01}V[\x80cI@K|\x11a\0\xDCW\x80cI@K|\x14a\x01\xE6W\x80cJ\xA4\xA4\xFC\x14a\x01\xF9W\x80c\x9B,\n7\x14a\x02\x1BW\x80c\xA4\xA7\x8F\x0C\x14a\x02.W\x80c\xAC\x96P\xD8\x14a\x02AWa\x01}V[\x80c\x12!\x0E\x8A\x14a\x01\x82W\x80c;\xEB&\xC4\x14a\x01\x8AW\x80cAK\xF3\x89\x14a\x01\xAAW\x80cFY\xA4\x94\x14a\x01\xD3Wa\x01}V[6a\x01}W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01{W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhNot WETH9`\xB8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\0[`\0\x80\xFD[a\x01{a\x03\x1BV[4\x80\x15a\x01\x96W`\0\x80\xFD[Pa\x01{a\x01\xA56`\x04a#cV[a\x03-V[a\x01\xBDa\x01\xB86`\x04a\"\\V[a\x032V[`@Qa\x01\xCA\x91\x90a%\xCDV[`@Q\x80\x91\x03\x90\xF3[a\x01{a\x01\xE16`\x04a\x1F\xDAV[a\x04YV[a\x01{a\x01\xF46`\x04a#{V[a\x04\xF3V[4\x80\x15a\x02\x05W`\0\x80\xFD[Pa\x02\x0Ea\x06mV[`@Qa\x01\xCA\x91\x90a$jV[a\x01{a\x02)6`\x04a#\xAAV[a\x06\x91V[a\x01{a\x02<6`\x04a\x1F\xDAV[a\x08]V[a\x02Ta\x02O6`\x04a :V[a\x08\xEEV[`@Qa\x01\xCA\x91\x90a$\xC4V[a\x01\xBDa\x02o6`\x04a!\xB1V[a\n.V[a\x01{a\x02\x826`\x04a\x1F\xDAV[a\x0BBV[4\x80\x15a\x02\x93W`\0\x80\xFD[Pa\x02\x0Ea\x0B\xD1V[a\x01\xBDa\x02\xAA6`\x04a\"\\V[a\x0B\xF5V[a\x01{a\x02\xBD6`\x04a\x1F;V[a\r\x1CV[a\x01{a\x02\xD06`\x04a\x1F|V[a\r\xFAV[a\x01\xBDa\x02\xE36`\x04a\"xV[a\x0F!V[a\x01{a\x02\xF66`\x04a\x1F\xDAV[a\x10\x17V[4\x80\x15a\x03\x07W`\0\x80\xFD[Pa\x01{a\x03\x166`\x04a \xCCV[a\x10\x89V[G\x15a\x03+Wa\x03+3Ga\x11\x9CV[V[`\x01UV[`\0\x81`\x80\x015\x80a\x03Ba\x12\x8BV[\x11\x15a\x03\x8BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x04$`\xA0\x84\x015a\x03\xA3`\x80\x86\x01``\x87\x01a\x1F\x18V[a\x03\xB4a\x01\0\x87\x01`\xE0\x88\x01a\x1F\x18V[`@\x80Q\x80\x82\x01\x90\x91R\x80a\x03\xCC` \x8A\x01\x8Aa\x1F\x18V[a\x03\xDC``\x8B\x01`@\x8C\x01a#@V[a\x03\xEC`@\x8C\x01` \x8D\x01a\x1F\x18V[`@Q` \x01a\x03\xFE\x93\x92\x91\x90a$\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x12\x91V[\x91P\x82`\xC0\x015\x82\x10\x15a\x04SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%cV[`@Q\x80\x91\x03\x90\xFD[P\x91\x90PV[`@\x80Qc#\xF2\xEB\xC3`\xE2\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\x01`\x84\x82\x01R`\xFF\x85\x16`\xA4\x82\x01R`\xC4\x81\x01\x84\x90R`\xE4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\x8F\xCB\xAF\x0C\x91a\x01\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x04\xD3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE7W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05bW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05vW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x05\x8CW`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x05\xDAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x06hW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06ZW=`\0\x80>=`\0\xFD[PPPPa\x06h\x82\x82a\x11\x9CV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x82\x11\x80\x15a\x06\xA2WP`d\x82\x11\x15[a\x06\xABW`\0\x80\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\x1AW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x07.W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x07DW`\0\x80\xFD[PQ\x90P\x84\x81\x10\x15a\x07\x92W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x08VW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x12W=`\0\x80>=`\0\xFD[PPPP`\0a'\x10a\x08.\x85\x84a\x13\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81a\x085W\xFE[\x04\x90P\x80\x15a\x08HWa\x08H\x83\x82a\x11\x9CV[a\x08T\x85\x82\x84\x03a\x11\x9CV[P[PPPPPV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q`\0\x19\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xAEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\xD8W`\0\x80\xFD[PQ\x10\x15a\x08TWa\x08T\x86\x86\x86\x86\x86\x86a\x04YV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\t\x07W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t;W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t&W\x90P[P\x90P`\0[\x82\x81\x10\x15a\n'W`\0\x800\x86\x86\x85\x81\x81\x10a\tYW\xFE[\x90P` \x02\x81\x01\x90a\tk\x91\x90a%\xD6V[`@Qa\ty\x92\x91\x90a$ZV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\t\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xB9V[``\x91P[P\x91P\x91P\x81a\n\x05W`D\x81Q\x10\x15a\t\xD2W`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\t\xEC\x91\x90a!GV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x91\x90a%$V[\x80\x84\x84\x81Q\x81\x10a\n\x12W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\tAV[P\x92\x91PPV[`\0\x81`@\x01Q\x80a\n>a\x12\x8BV[\x11\x15a\n\x87W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[3[`\0a\n\x98\x85`\0\x01Qa\x14\rV[\x90Pa\n\xE4\x85``\x01Q\x82a\n\xB1W\x86` \x01Qa\n\xB3V[0[`\0`@Q\x80`@\x01`@R\x80a\n\xCD\x8B`\0\x01Qa\x14\x19V[\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x12\x91V[``\x86\x01R\x80\x15a\x0B\x04W\x84Q0\x92Pa\n\xFD\x90a\x14(V[\x85Ra\x0B\x11V[\x84``\x01Q\x93PPa\x0B\x17V[Pa\n\x89V[\x83`\x80\x01Q\x83\x10\x15a\x0B;W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%cV[PP\x91\x90PV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q\x86\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0B\x91W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0B\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0B\xBBW`\0\x80\xFD[PQ\x10\x15a\x08TWa\x08T\x86\x86\x86\x86\x86\x86a\x10\x17V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x81`\x80\x015\x80a\x0C\x05a\x12\x8BV[\x11\x15a\x0CNW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0C\xEA`\xA0\x84\x015a\x0Cf`\x80\x86\x01``\x87\x01a\x1F\x18V[a\x0Cwa\x01\0\x87\x01`\xE0\x88\x01a\x1F\x18V[`@Q\x80`@\x01`@R\x80\x88` \x01` \x81\x01\x90a\x0C\x95\x91\x90a\x1F\x18V[a\x0C\xA5``\x8B\x01`@\x8C\x01a#@V[a\x0C\xB2` \x8C\x01\x8Ca\x1F\x18V[`@Q` \x01a\x0C\xC4\x93\x92\x91\x90a$\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x14?V[\x91P\x82`\xC0\x015\x82\x11\x15a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%7V[P`\0\x19`\0U\x91\x90PV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\rkW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\r\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\r\x95W`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\r\xE3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\r\xF4Wa\r\xF4\x84\x83\x83a\x15\xBAV[PPPPV[`\0\x82\x11\x80\x15a\x0E\x0BWP`d\x82\x11\x15[a\x0E\x14W`\0\x80\xFD[`\0\x85`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0EcW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0EwW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E\x8DW`\0\x80\xFD[PQ\x90P\x84\x81\x10\x15a\x0E\xDBW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x08TW`\0a'\x10a\x0E\xF0\x83\x86a\x13\xE3V[\x81a\x0E\xF7W\xFE[\x04\x90P\x80\x15a\x0F\x0BWa\x0F\x0B\x87\x84\x83a\x15\xBAV[a\x0F\x18\x87\x86\x83\x85\x03a\x15\xBAV[PPPPPPPV[`\0\x81`@\x015\x80a\x0F1a\x12\x8BV[\x11\x15a\x0FzW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x0F\xED``\x84\x015a\x0F\x92`@\x86\x01` \x87\x01a\x1F\x18V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x90\x80a\x0F\xAA\x89\x80a%\xD6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP3` \x90\x91\x01Ra\x14?V[P`\0T\x91P\x82`\x80\x015\x82\x11\x15a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04J\x90a%7V[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\xFF\x85\x16`\x84\x82\x01R`\xA4\x81\x01\x84\x90R`\xC4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x04\xD3W`\0\x80\xFD[`\0\x84\x13\x80a\x10\x98WP`\0\x83\x13[a\x10\xA1W`\0\x80\xFD[`\0a\x10\xAF\x82\x84\x01\x84a\"\xB0V[\x90P`\0\x80`\0a\x10\xC3\x84`\0\x01Qa\x17\x01V[\x92P\x92P\x92Pa\x10\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\x172V[P`\0\x80`\0\x8A\x13a\x11\x1CW\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10\x89a\x113V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x8A[\x91P\x91P\x81\x15a\x11RWa\x11M\x85\x87` \x01Q3\x84a\x17QV[a\x04\xE7V[\x85Qa\x11]\x90a\x14\rV[\x15a\x11\x82W\x85Qa\x11m\x90a\x14(V[\x86Ra\x11|\x813`\0\x89a\x14?V[Pa\x04\xE7V[\x80`\0\x81\x90UP\x83\x94Pa\x04\xE7\x85\x87` \x01Q3\x84a\x17QV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x11\xE8W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x11\xC9V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12JW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12OV[``\x91P[PP\x90P\x80a\x06hW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01T\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16a\x12\xA5W0\x93P[`\0\x80`\0a\x12\xB7\x85`\0\x01Qa\x17\x01V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x90\x84\x16\x10`\0\x80a\x12\xDB\x86\x86\x86a\x18\xE1V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x8B\x85a\x12\xF4\x8Fa\x19\x1FV[`\x01`\x01`\xA0\x1B\x03\x8E\x16\x15a\x13\tW\x8Da\x13/V[\x87a\x13(Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x13/V[d\x01\0\x02v\xA4[\x8D`@Q` \x01a\x13@\x91\x90a%\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13o\x95\x94\x93\x92\x91\x90a$~V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xC0\x91\x90a \xA9V[\x91P\x91P\x82a\x13\xCFW\x81a\x13\xD1V[\x80[`\0\x03\x9B\x9APPPPPPPPPPPV[`\0\x82\x15\x80a\x13\xFEWPP\x81\x81\x02\x81\x83\x82\x81a\x13\xFBW\xFE[\x04\x14[a\x14\x07W`\0\x80\xFD[\x92\x91PPV[\x80Q`B\x11\x15[\x91\x90PV[``a\x14\x07\x82`\0`+a\x195V[\x80Q``\x90a\x14\x07\x90\x83\x90`\x17\x90`\x16\x19\x01a\x195V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16a\x14SW0\x93P[`\0\x80`\0a\x14e\x85`\0\x01Qa\x17\x01V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10`\0\x80a\x14\x89\x85\x87\x86a\x18\xE1V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x8B\x85a\x14\xA2\x8Fa\x19\x1FV[`\0\x03`\x01`\x01`\xA0\x1B\x03\x8E\x16\x15a\x14\xBAW\x8Da\x14\xE0V[\x87a\x14\xD9Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x14\xE0V[d\x01\0\x02v\xA4[\x8D`@Q` \x01a\x14\xF1\x91\x90a%\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15 \x95\x94\x93\x92\x91\x90a$~V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x159W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15q\x91\x90a \xA9V[\x91P\x91P`\0\x83a\x15\x86W\x81\x83`\0\x03a\x15\x8CV[\x82\x82`\0\x03[\x90\x98P\x90P`\x01`\x01`\xA0\x1B\x03\x8A\x16a\x15\xABW\x8B\x81\x14a\x15\xABW`\0\x80\xFD[PPPPPPP\x94\x93PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x166W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x16\x17V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x16\x98W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x16\x9DV[``\x91P[P\x91P\x91P\x81\x80\x15a\x16\xCBWP\x80Q\x15\x80a\x16\xCBWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x16\xC8W`\0\x80\xFD[PQ[a\x08VW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x80a\x17\x0F\x84\x82a\x1A\x86V[\x92Pa\x17\x1C\x84`\x14a\x1B6V[\x90Pa\x17)\x84`\x17a\x1A\x86V[\x91P\x91\x93\x90\x92PV[`\0a\x17H\x85a\x17C\x86\x86\x86a\x1B\xDDV[a\x1C3V[\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\x17\x92WP\x80G\x10\x15[\x15a\x18\xB4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x17\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x06W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x18\xACW`\0\x80\xFD[Pa\r\xF4\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14\x15a\x18\xD5Wa\x18\xD0\x84\x83\x83a\x15\xBAV[a\r\xF4V[a\r\xF4\x84\x84\x84\x84a\x1CVV[`\0a\x19\x17\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x19\x12\x86\x86\x86a\x1B\xDDV[a\x1D\xA6V[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\x191W`\0\x80\xFD[P\x90V[``\x81\x82`\x1F\x01\x10\x15a\x19\x80W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82\x82\x84\x01\x10\x15a\x19\xC8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81\x83\x01\x84Q\x10\x15a\x1A\x14W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[``\x82\x15\x80\x15a\x1A3W`@Q\x91P`\0\x82R` \x82\x01`@Ra\x1A}V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x1AlW\x80Q\x83R` \x92\x83\x01\x92\x01a\x1ATV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[`\0\x81\x82`\x14\x01\x10\x15a\x1A\xD5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x14\x01\x83Q\x10\x15a\x1B&W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81\x82`\x03\x01\x10\x15a\x1B\x84W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x03\x01\x83Q\x10\x15a\x1B\xD4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01`\x03\x01Q\x90V[a\x1B\xE5a\x1E\x8AV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x1C\x03W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0a\x1C?\x83\x83a\x1D\xA6V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x14\x07W`\0\x80\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x1C\xDAW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x1C\xBBV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1D<W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1DAV[``\x91P[P\x91P\x91P\x81\x80\x15a\x1DoWP\x80Q\x15\x80a\x1DoWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x1DlW`\0\x80\xFD[PQ[a\x08TW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb)\xAA#`\xE9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x1D\xCEW`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[\x805a\x14\x14\x81a&\x94V[`\0\x82`\x1F\x83\x01\x12a\x1E\xC5W\x80\x81\xFD[\x815a\x1E\xD8a\x1E\xD3\x82a&FV[a&\"V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E\xECW\x82\x83\xFD[\x81` \x85\x01` \x83\x017\x90\x81\x01` \x01\x91\x90\x91R\x92\x91PPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x04SW\x80\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x1F)W\x80\x81\xFD[\x815a\x1F4\x81a&\x94V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1FOW\x81\x82\xFD[\x835a\x1FZ\x81a&\x94V[\x92P` \x84\x015\x91P`@\x84\x015a\x1Fq\x81a&\x94V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1F\x93W\x80\x81\xFD[\x855a\x1F\x9E\x81a&\x94V[\x94P` \x86\x015\x93P`@\x86\x015a\x1F\xB5\x81a&\x94V[\x92P``\x86\x015\x91P`\x80\x86\x015a\x1F\xCC\x81a&\x94V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xF2W\x80\x81\xFD[\x865a\x1F\xFD\x81a&\x94V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a  W\x81\x82\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a LW\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a cW\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a vW\x83\x84\xFD[\x815\x81\x81\x11\x15a \x84W\x84\x85\xFD[\x86` \x80\x83\x02\x85\x01\x01\x11\x15a \x97W\x84\x85\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a \xBBW\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a \xE1W\x81\x82\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\x06W\x83\x84\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a!\x19W\x83\x84\xFD[\x815\x81\x81\x11\x15a!'W\x84\x85\xFD[\x88` \x82\x85\x01\x01\x11\x15a!8W\x84\x85\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a!XW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!nW\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13a!~W\x81\x82\xFD[\x80Qa!\x8Ca\x1E\xD3\x82a&FV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a!\xA0W\x83\x84\xFD[a\x17H\x82` \x83\x01` \x86\x01a&hV[`\0` \x82\x84\x03\x12\x15a!\xC2W\x80\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xD9W\x82\x83\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a!\xECW\x82\x83\xFD[`@Q`\xA0\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\"\x01W\xFE[`@R\x825\x82\x81\x11\x15a\"\x12W\x84\x85\xFD[a\"\x1E\x87\x82\x86\x01a\x1E\xB5V[\x82RPa\"-` \x84\x01a\x1E\xAAV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0a\x01\0\x82\x84\x03\x12\x15a\"nW\x80\x81\xFD[a\x1F4\x83\x83a\x1F\x06V[`\0` \x82\x84\x03\x12\x15a\"\x89W\x80\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x9FW\x81\x82\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x1F4W\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15a\"\xC1W\x80\x81\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD8W\x82\x83\xFD[\x90\x83\x01\x90`@\x82\x86\x03\x12\x15a\"\xEBW\x82\x83\xFD[`@Q`@\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a#\0W\xFE[`@R\x825\x82\x81\x11\x15a#\x11W\x84\x85\xFD[a#\x1D\x87\x82\x86\x01a\x1E\xB5V[\x82RP` \x83\x015\x92Pa#0\x83a&\x94V[` \x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a#QW\x80\x81\xFD[\x815b\xFF\xFF\xFF\x81\x16\x81\x14a\x1F4W\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15a#tW\x80\x81\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\x8DW\x81\x82\xFD[\x825\x91P` \x83\x015a#\x9F\x81a&\x94V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a#\xBFW\x81\x82\xFD[\x845\x93P` \x85\x015a#\xD1\x81a&\x94V[\x92P`@\x85\x015\x91P``\x85\x015a#\xE8\x81a&\x94V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x81Q\x80\x84Ra$\x0B\x81` \x86\x01` \x86\x01a&hV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\0\x82\x84\x837\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a$\xB9\x90\x83\x01\x84a#\xF3V[\x97\x96PPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x84\x82\x02\x87\x01\x01\x92P\x83\x87\x01\x85[\x82\x81\x10\x15a%\x17W`?\x19\x88\x86\x03\x01\x84Ra%\x05\x85\x83Qa#\xF3V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a$\xE9V[P\x92\x97\x96PPPPPPPV[`\0` \x82Ra\x1F4` \x83\x01\x84a#\xF3V[` \x80\x82R`\x12\x90\x82\x01Rq\x15\x1B\xDB\xC8\x1B]X\xDA\x08\x1C\x99\\]Y\\\xDD\x19Y`r\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x13\x90\x82\x01Rr\x15\x1B\xDB\xC8\x1B\x1A]\x1D\x1B\x19H\x1C\x99X\xD9Z]\x99Y`j\x1B`@\x82\x01R``\x01\x90V[`\0` \x82R\x82Q`@` \x84\x01Ra%\xAC``\x84\x01\x82a#\xF3V[` \x94\x90\x94\x01Q`\x01`\x01`\xA0\x1B\x03\x16`@\x93\x90\x93\x01\x92\x90\x92RP\x90\x91\x90PV[\x90\x81R` \x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a%\xECW\x82\x83\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\x06W\x82\x83\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a&\x1BW`\0\x80\xFD[\x92P\x92\x90PV[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&>W\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&ZW\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a&\x83W\x81\x81\x01Q\x83\x82\x01R` \x01a&kV[\x83\x81\x11\x15a\r\xF4WPP`\0\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a&\xA9W`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 \x16\xD8\x9D\xDA7'R,\x15\xE0\x97\x88\xCF\xB0\xE1\xD9\x07\xEEc\xFEv\xAC-\xE0\x11\xC0Ov\xBB\xC9\xE2(dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKTIMESWAPROUTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockTimeSwapRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockTimeSwapRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockTimeSwapRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockTimeSwapRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockTimeSwapRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockTimeSwapRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockTimeSwapRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKTIMESWAPROUTER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKTIMESWAPROUTER_ABI.clone(),
                MOCKTIMESWAPROUTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInput` (0xc04b8d59) function
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 75, 141, 89], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInputSingle` (0x414bf389) function
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 75, 243, 137], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactOutput` (0xf28c0498) function
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 140, 4, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactOutputSingle` (0xdb3e2198) function
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 62, 33, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundETH` (0x12210e8a) function
        pub fn refund_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermit` (0xf3995c67) function
        pub fn self_permit(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowed` (0x4659a494) function
        pub fn self_permit_allowed(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function
        pub fn self_permit_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTime` (0x3beb26c4) function
        pub fn set_time(
            &self,
            time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 235, 38, 196], time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepToken` (0xdf2ab5bb) function
        pub fn sweep_token(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepTokenWithFee` (0xe0e189a0) function
        pub fn sweep_token_with_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 225, 137, 160],
                    (token, amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9` (0x49404b7c) function
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9WithFee` (0x9b2c0a37) function
        pub fn unwrap_weth9_with_fee(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 44, 10, 55],
                    (amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockTimeSwapRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    ///Container type for all input parameters for the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `0xc04b8d59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "exactInput",
        abi = "exactInput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    ///Container type for all input parameters for the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0x414bf389`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    ///Container type for all input parameters for the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `0xf28c0498`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "exactOutput",
        abi = "exactOutput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }
    ///Container type for all input parameters for the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0xdb3e2198`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `refundETH` function with signature `refundETH()` and selector `0x12210e8a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
    ///Container type for all input parameters for the `selfPermit` function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3995c67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowed` function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x4659a494`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowedIfNecessary` function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xa4a78f0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitIfNecessary` function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc2e3140a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `setTime` function with signature `setTime(uint256)` and selector `0x3beb26c4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTime", abi = "setTime(uint256)")]
    pub struct SetTimeCall {
        pub time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sweepToken` function with signature `sweepToken(address,uint256,address)` and selector `0xdf2ab5bb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sweepTokenWithFee` function with signature `sweepTokenWithFee(address,uint256,address,uint256,address)` and selector `0xe0e189a0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sweepTokenWithFee",
        abi = "sweepTokenWithFee(address,uint256,address,uint256,address)"
    )]
    pub struct SweepTokenWithFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256,address)` and selector `0x49404b7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH9WithFee` function with signature `unwrapWETH9WithFee(uint256,address,uint256,address)` and selector `0x9b2c0a37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "unwrapWETH9WithFee",
        abi = "unwrapWETH9WithFee(uint256,address,uint256,address)"
    )]
    pub struct UnwrapWETH9WithFeeCall {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MockTimeSwapRouterCalls {
        Weth9(Weth9Call),
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        Factory(FactoryCall),
        Multicall(MulticallCall),
        RefundETH(RefundETHCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        SetTime(SetTimeCall),
        SweepToken(SweepTokenCall),
        SweepTokenWithFee(SweepTokenWithFeeCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
        UnwrapWETH9(UnwrapWETH9Call),
        UnwrapWETH9WithFee(UnwrapWETH9WithFeeCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockTimeSwapRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <ExactInputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExactInput(decoded));
            }
            if let Ok(decoded) = <ExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExactInputSingle(decoded));
            }
            if let Ok(decoded) = <ExactOutputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExactOutput(decoded));
            }
            if let Ok(decoded) = <ExactOutputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExactOutputSingle(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <RefundETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RefundETH(decoded));
            }
            if let Ok(decoded) = <SelfPermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermit(decoded));
            }
            if let Ok(decoded) = <SelfPermitAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) = <SelfPermitAllowedIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) = <SelfPermitIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermitIfNecessary(decoded));
            }
            if let Ok(decoded) = <SetTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTime(decoded));
            }
            if let Ok(decoded) = <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded) = <SweepTokenWithFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SweepTokenWithFee(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9WithFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnwrapWETH9WithFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockTimeSwapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExactInputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExactOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExactOutputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RefundETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepTokenWithFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9WithFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockTimeSwapRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactOutputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepTokenWithFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9WithFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for MockTimeSwapRouterCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<ExactInputCall> for MockTimeSwapRouterCalls {
        fn from(value: ExactInputCall) -> Self {
            Self::ExactInput(value)
        }
    }
    impl ::core::convert::From<ExactInputSingleCall> for MockTimeSwapRouterCalls {
        fn from(value: ExactInputSingleCall) -> Self {
            Self::ExactInputSingle(value)
        }
    }
    impl ::core::convert::From<ExactOutputCall> for MockTimeSwapRouterCalls {
        fn from(value: ExactOutputCall) -> Self {
            Self::ExactOutput(value)
        }
    }
    impl ::core::convert::From<ExactOutputSingleCall> for MockTimeSwapRouterCalls {
        fn from(value: ExactOutputSingleCall) -> Self {
            Self::ExactOutputSingle(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for MockTimeSwapRouterCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for MockTimeSwapRouterCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<RefundETHCall> for MockTimeSwapRouterCalls {
        fn from(value: RefundETHCall) -> Self {
            Self::RefundETH(value)
        }
    }
    impl ::core::convert::From<SelfPermitCall> for MockTimeSwapRouterCalls {
        fn from(value: SelfPermitCall) -> Self {
            Self::SelfPermit(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedCall> for MockTimeSwapRouterCalls {
        fn from(value: SelfPermitAllowedCall) -> Self {
            Self::SelfPermitAllowed(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedIfNecessaryCall>
    for MockTimeSwapRouterCalls {
        fn from(value: SelfPermitAllowedIfNecessaryCall) -> Self {
            Self::SelfPermitAllowedIfNecessary(value)
        }
    }
    impl ::core::convert::From<SelfPermitIfNecessaryCall> for MockTimeSwapRouterCalls {
        fn from(value: SelfPermitIfNecessaryCall) -> Self {
            Self::SelfPermitIfNecessary(value)
        }
    }
    impl ::core::convert::From<SetTimeCall> for MockTimeSwapRouterCalls {
        fn from(value: SetTimeCall) -> Self {
            Self::SetTime(value)
        }
    }
    impl ::core::convert::From<SweepTokenCall> for MockTimeSwapRouterCalls {
        fn from(value: SweepTokenCall) -> Self {
            Self::SweepToken(value)
        }
    }
    impl ::core::convert::From<SweepTokenWithFeeCall> for MockTimeSwapRouterCalls {
        fn from(value: SweepTokenWithFeeCall) -> Self {
            Self::SweepTokenWithFee(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for MockTimeSwapRouterCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for MockTimeSwapRouterCalls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9WithFeeCall> for MockTimeSwapRouterCalls {
        fn from(value: UnwrapWETH9WithFeeCall) -> Self {
            Self::UnwrapWETH9WithFee(value)
        }
    }
    ///Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Weth9Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `0xc04b8d59`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0x414bf389`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `0xf28c0498`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExactOutputReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `0xdb3e2198`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExactOutputSingleReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
}
