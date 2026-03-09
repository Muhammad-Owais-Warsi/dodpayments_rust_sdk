use crate::{
    client::DodoPaymentsClient,
    models::{
        ChangePlanPreviewResponse, CreateSubscriptionChargeRequest,
        CreateSubscriptionChargeResponse, CreateSubscriptionRequest, CreateSubscriptionResponse,
        GetSubscriptionsListResponse, ListUsageHistoryQueryParams, ListUsageHistoryResponse,
        PatchSubscriptionRequest, SubscriptionResponse, UpdatePaymentMethodReq,
        UpdatePaymentMethodResponse, UpdateSubscriptionPlanReq,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct SubscriptionsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> SubscriptionsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetSubscriptionsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/subscriptions")
    }

    #[deprecated(note = "This API will be deprecated soon. We recommend using Checkout Sessions.")]
    pub fn create(
        &self,
    ) -> RequestBuilder<'client, CreateSubscriptionResponse, (), CreateSubscriptionRequest> {
        RequestBuilder::new(self.client, Method::POST, "/subscriptions")
    }

    pub fn id(&self, subscription_id: impl Into<String>) -> SubscriptionByIdApi<'client> {
        SubscriptionByIdApi {
            client: self.client,
            subscription_id: subscription_id.into(),
        }
    }
}

pub struct SubscriptionByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    subscription_id: String,
}

impl<'client> SubscriptionByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, SubscriptionResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/subscriptions/{}", self.subscription_id),
        )
    }

    pub fn update(
        &self,
    ) -> RequestBuilder<'client, SubscriptionResponse, (), PatchSubscriptionRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/subscriptions/{}", self.subscription_id),
        )
    }

    pub fn update_payment_method(
        &self,
    ) -> RequestBuilder<'client, UpdatePaymentMethodResponse, UpdatePaymentMethodReq> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!(
                "/subscriptions/{}/update-payment-method",
                self.subscription_id
            ),
        )
    }

    pub fn charge(
        &self,
    ) -> RequestBuilder<
        'client,
        CreateSubscriptionChargeResponse,
        (),
        CreateSubscriptionChargeRequest,
    > {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/subscriptions/{}/charge", self.subscription_id),
        )
    }

    pub fn change_plan(
        &self,
    ) -> RequestBuilder<'client, SubscriptionResponse, (), UpdateSubscriptionPlanReq> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/subscriptions/{}/change-plan", self.subscription_id),
        )
    }

    pub fn preview_plan_change(
        &self,
    ) -> RequestBuilder<'client, ChangePlanPreviewResponse, (), UpdateSubscriptionPlanReq> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!(
                "/subscriptions/{}/change-plan/preview",
                self.subscription_id,
            ),
        )
    }

    pub fn usage_history(
        &self,
    ) -> RequestBuilder<'client, ListUsageHistoryResponse, ListUsageHistoryQueryParams, ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/subscriptions/{}/usage-history", self.subscription_id),
        )
    }
}
