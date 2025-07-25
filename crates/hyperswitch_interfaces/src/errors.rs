//! Errors interface

use common_enums::ApiClientError;
use common_utils::errors::ErrorSwitch;
use hyperswitch_domain_models::errors::api_error_response::ApiErrorResponse;

/// Connector Errors
#[allow(missing_docs, missing_debug_implementations)]
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ConnectorError {
    #[error("Error while obtaining URL for the integration")]
    FailedToObtainIntegrationUrl,
    #[error("Failed to encode connector request")]
    RequestEncodingFailed,
    #[error("Request encoding failed : {0}")]
    RequestEncodingFailedWithReason(String),
    #[error("Parsing failed")]
    ParsingFailed,
    #[error("Failed to deserialize connector response")]
    ResponseDeserializationFailed,
    #[error("Failed to execute a processing step: {0:?}")]
    ProcessingStepFailed(Option<bytes::Bytes>),
    #[error("The connector returned an unexpected response: {0:?}")]
    UnexpectedResponseError(bytes::Bytes),
    #[error("Failed to parse custom routing rules from merchant account")]
    RoutingRulesParsingError,
    #[error("Failed to obtain preferred connector from merchant account")]
    FailedToObtainPreferredConnector,
    #[error("An invalid connector name was provided")]
    InvalidConnectorName,
    #[error("An invalid Wallet was used")]
    InvalidWallet,
    #[error("Failed to handle connector response")]
    ResponseHandlingFailed,
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: &'static str },
    #[error("Missing required fields: {field_names:?}")]
    MissingRequiredFields { field_names: Vec<&'static str> },
    #[error("Failed to obtain authentication type")]
    FailedToObtainAuthType,
    #[error("Failed to obtain certificate")]
    FailedToObtainCertificate,
    #[error("Connector meta data not found")]
    NoConnectorMetaData,
    #[error("Connector wallet details not found")]
    NoConnectorWalletDetails,
    #[error("Failed to obtain certificate key")]
    FailedToObtainCertificateKey,
    #[error("This step has not been implemented for: {0}")]
    NotImplemented(String),
    #[error("{message} is not supported by {connector}")]
    NotSupported {
        message: String,
        connector: &'static str,
    },
    #[error("{flow} flow not supported by {connector} connector")]
    FlowNotSupported { flow: String, connector: String },
    #[error("Capture method not supported")]
    CaptureMethodNotSupported,
    #[error("Missing connector mandate ID")]
    MissingConnectorMandateID,
    #[error("Missing connector mandate metadata")]
    MissingConnectorMandateMetadata,
    #[error("Missing connector transaction ID")]
    MissingConnectorTransactionID,
    #[error("Missing connector refund ID")]
    MissingConnectorRefundID,
    #[error("Missing apple pay tokenization data")]
    MissingApplePayTokenData,
    #[error("Webhooks not implemented for this connector")]
    WebhooksNotImplemented,
    #[error("Failed to decode webhook event body")]
    WebhookBodyDecodingFailed,
    #[error("Signature not found for incoming webhook")]
    WebhookSignatureNotFound,
    #[error("Failed to verify webhook source")]
    WebhookSourceVerificationFailed,
    #[error("Could not find merchant secret in DB for incoming webhook source verification")]
    WebhookVerificationSecretNotFound,
    #[error("Merchant secret found for incoming webhook source verification is invalid")]
    WebhookVerificationSecretInvalid,
    #[error("Incoming webhook object reference ID not found")]
    WebhookReferenceIdNotFound,
    #[error("Incoming webhook event type not found")]
    WebhookEventTypeNotFound,
    #[error("Incoming webhook event resource object not found")]
    WebhookResourceObjectNotFound,
    #[error("Could not respond to the incoming webhook event")]
    WebhookResponseEncodingFailed,
    #[error("Invalid Date/time format")]
    InvalidDateFormat,
    #[error("Date Formatting Failed")]
    DateFormattingFailed,
    #[error("Invalid Data format")]
    InvalidDataFormat { field_name: &'static str },
    #[error("Payment Method data / Payment Method Type / Payment Experience Mismatch ")]
    MismatchedPaymentData,
    #[error("Failed to parse {wallet_name} wallet token")]
    InvalidWalletToken { wallet_name: String },
    #[error("Missing Connector Related Transaction ID")]
    MissingConnectorRelatedTransactionID { id: String },
    #[error("File Validation failed")]
    FileValidationFailed { reason: String },
    #[error("Missing 3DS redirection payload: {field_name}")]
    MissingConnectorRedirectionPayload { field_name: &'static str },
    #[error("Failed at connector's end with code '{code}'")]
    FailedAtConnector { message: String, code: String },
    #[error("Payment Method Type not found")]
    MissingPaymentMethodType,
    #[error("Balance in the payment method is low")]
    InSufficientBalanceInPaymentMethod,
    #[error("Server responded with Request Timeout")]
    RequestTimeoutReceived,
    #[error("The given currency method is not configured with the given connector")]
    CurrencyNotSupported {
        message: String,
        connector: &'static str,
    },
    #[error("Invalid Configuration")]
    InvalidConnectorConfig { config: &'static str },
    #[error("Failed to convert amount to required type")]
    AmountConversionFailed,
    #[error("Generic Error")]
    GenericError {
        error_message: String,
        error_object: serde_json::Value,
    },
    #[error("Field {fields} doesn't match with the ones used during mandate creation")]
    MandatePaymentDataMismatch { fields: String },
}

impl ConnectorError {
    /// fn is_connector_timeout
    pub fn is_connector_timeout(&self) -> bool {
        self == &Self::RequestTimeoutReceived
    }
}

impl ErrorSwitch<ConnectorError> for common_utils::errors::ParsingError {
    fn switch(&self) -> ConnectorError {
        ConnectorError::ParsingFailed
    }
}

impl ErrorSwitch<ApiErrorResponse> for ConnectorError {
    fn switch(&self) -> ApiErrorResponse {
        match self {
            Self::WebhookSourceVerificationFailed => ApiErrorResponse::WebhookAuthenticationFailed,
            Self::WebhookSignatureNotFound
            | Self::WebhookReferenceIdNotFound
            | Self::WebhookResourceObjectNotFound
            | Self::WebhookBodyDecodingFailed
            | Self::WebhooksNotImplemented => ApiErrorResponse::WebhookBadRequest,
            Self::WebhookEventTypeNotFound => ApiErrorResponse::WebhookUnprocessableEntity,
            Self::WebhookVerificationSecretInvalid => {
                ApiErrorResponse::WebhookInvalidMerchantSecret
            }
            _ => ApiErrorResponse::InternalServerError,
        }
    }
}

// http client errors
#[allow(missing_docs, missing_debug_implementations)]
#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum HttpClientError {
    #[error("Header map construction failed")]
    HeaderMapConstructionFailed,
    #[error("Invalid proxy configuration")]
    InvalidProxyConfiguration,
    #[error("Client construction failed")]
    ClientConstructionFailed,
    #[error("Certificate decode failed")]
    CertificateDecodeFailed,
    #[error("Request body serialization failed")]
    BodySerializationFailed,
    #[error("Unexpected state reached/Invariants conflicted")]
    UnexpectedState,

    #[error("Failed to parse URL")]
    UrlParsingFailed,
    #[error("URL encoding of request payload failed")]
    UrlEncodingFailed,
    #[error("Failed to send request to connector {0}")]
    RequestNotSent(String),
    #[error("Failed to decode response")]
    ResponseDecodingFailed,

    #[error("Server responded with Request Timeout")]
    RequestTimeoutReceived,

    #[error("connection closed before a message could complete")]
    ConnectionClosedIncompleteMessage,

    #[error("Server responded with Internal Server Error")]
    InternalServerErrorReceived,
    #[error("Server responded with Bad Gateway")]
    BadGatewayReceived,
    #[error("Server responded with Service Unavailable")]
    ServiceUnavailableReceived,
    #[error("Server responded with Gateway Timeout")]
    GatewayTimeoutReceived,
    #[error("Server responded with unexpected response")]
    UnexpectedServerResponse,
}

impl ErrorSwitch<ApiClientError> for HttpClientError {
    fn switch(&self) -> ApiClientError {
        match self {
            Self::HeaderMapConstructionFailed => ApiClientError::HeaderMapConstructionFailed,
            Self::InvalidProxyConfiguration => ApiClientError::InvalidProxyConfiguration,
            Self::ClientConstructionFailed => ApiClientError::ClientConstructionFailed,
            Self::CertificateDecodeFailed => ApiClientError::CertificateDecodeFailed,
            Self::BodySerializationFailed => ApiClientError::BodySerializationFailed,
            Self::UnexpectedState => ApiClientError::UnexpectedState,
            Self::UrlParsingFailed => ApiClientError::UrlParsingFailed,
            Self::UrlEncodingFailed => ApiClientError::UrlEncodingFailed,
            Self::RequestNotSent(reason) => ApiClientError::RequestNotSent(reason.clone()),
            Self::ResponseDecodingFailed => ApiClientError::ResponseDecodingFailed,
            Self::RequestTimeoutReceived => ApiClientError::RequestTimeoutReceived,
            Self::ConnectionClosedIncompleteMessage => {
                ApiClientError::ConnectionClosedIncompleteMessage
            }
            Self::InternalServerErrorReceived => ApiClientError::InternalServerErrorReceived,
            Self::BadGatewayReceived => ApiClientError::BadGatewayReceived,
            Self::ServiceUnavailableReceived => ApiClientError::ServiceUnavailableReceived,
            Self::GatewayTimeoutReceived => ApiClientError::GatewayTimeoutReceived,
            Self::UnexpectedServerResponse => ApiClientError::UnexpectedServerResponse,
        }
    }
}
