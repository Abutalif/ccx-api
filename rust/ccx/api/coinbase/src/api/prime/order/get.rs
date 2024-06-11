use crate::api::prime::prelude::*;
use crate::api::prime::AccountPortfolioOrder;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioOrderResponse {
    pub order: AccountPortfolioOrder,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Get Order by Order ID.
    ///
    /// Retrieve an order by order ID.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The ID of the portfolio under which the order was placed.
    /// * `order_id` - The order ID generated by Coinbase upon order submission.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getorder]
    pub fn get_order(
        &self,
        portfolio_id: Uuid,
        order_id: Uuid,
    ) -> CoinbaseResult<Task<AccountPortfolioOrderResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/orders/{order_id}");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
