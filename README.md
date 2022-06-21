# LFICO Protocol - License-Free Initial Coin Offering Protocol

LFICO Protocol is a framework for users to offer digital assets without any licenses. Exchanges might have a license but offering tokens itself can be done by anyone thanks to [Generalized DEX](https://github.com/kentomisawa/generalized-dex), and [Sulfur Network](https://github.com/kentomisawa/sulfur) helps to corporate with regulated entities seamlessly.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

## The Reason Why License-Free

This is because the main reasons why countries regulate crypto are protection for users' assets and Anti-Money Laundering/Combating the Financing of Terrorism (AML/CFT). Without these two reasons, countries don't have much reason to regulate because the disadvantages surpass the benefits. LFICO Protocol offers a scaffold to avoid the risks of AML/CFT and loss of assets.

### How To Avoid Asset Loss

[Generalized DEX](https://github.com/kentomisawa/generalized-dex) has the value-stable token pool, and token issuers can set how much of the paid value for the purchase of the tokens is backed in the pool. The backed values are token agnostic and volatility-resistant over time; even stabler than any fiat currencies. In addition to it, the pricing logic is customizable unless the withdrawal value exceeds the deposit. Therefore, token issuers can avoid the loss of assets owned by users.

### How To Avoid AML/CFT

Just restrict exchange and transferrence following the regulation in the target jurisdiction. Like, restricting until an address follows Financial Action Task Force (FATF)'s Travel Rule is one example. Offering untransferrable tokens is the same as selling a product. The problem with the existing tokens is the ability of anonymous transferrence. If it's not able to transfer anonymously, there is no reason to be regulated by AML/CFT.

#### Compliance And Verification Sharing

Thanks to [Sulfur Network](https://github.com/kentomisawa/sulfur), it is easy to share the states of verification and regulations of a specific jurisdiction.

### Pricing Customization

Pricing Customization might be a little tricky. Because it should be dynamic pricing rather than the pricing of derivatives, and the definition of the derivatives are various to each country. If other users' purchase and selling behaviors directly affect on the price, the products could be regarded as derivatives. So, to ensure the products are not derivatives, the token offerors can make the pricing coupled with external systems other than the purchase and selling power directly. However, it is more important to focus on the solution for underlying issues on derivatives rather than nominal definitions. Because if there are still issues, the tokens should be regulated with different rules. Therefore, at least, pricing and value backing have to be safe enough to fairly protect users' assets.

### Fundraising

It's up to users how to fundraise with token offerings. But maybe a popular way is to withdraw a part of the backed value from the pool. For example, 80% of paid values are stored in the pool, then 20% of them are withdrawn by the token issuers as funds. Another way is that all of the paid values are stored in the pool for custom pricing, but traders have to pay margin fees to the token issuers.

### Transferability

Transferability must be restricted at first because the definition of the derivatives is various for each country. The ability to exchange other assets may be one of the definitions. So the token offerors should also consider how to exchange into other assets, or, they can prohibit exchanging or transferring at all. But again, solving the fundamental problems is more important than nominal definitions to make it sustainable. In this case that is to make sure the flow of assets is safe and fair.  
Even if custom tokens are not transferrable, users can exchange those tokens with other assets because custom tokens are burnt and minted when deposited and withdrawn. The safest way to do license-free ICO is to offer tokens that anonymous users are prohibited from exchanging or transferring and with dynamic pricing that will not cause unexpected huge asset loss. Due to the nature of burning and minting, there is no difference between offering and exchanging.

### The Flow

Token issuers set the target jurisdiction in which they offer tokens to users. Part of paid amounts is withdrawn by the token issuers. Users cannot transfer tokens at first. Then, an integrated third party like an exchange checks KYC or other compliance and verifies the address to transfer or exchange.
