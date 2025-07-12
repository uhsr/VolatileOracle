# VolatileOracle: Decentralized Threshold-Triggered Crypto Price Alerts

A serverless and verifiable system for receiving crypto price alerts based on pre-defined volatility thresholds, powered by smart contracts and on-chain oracles.

VolatileOracle provides a decentralized mechanism for users to set up alerts that trigger when the price of a cryptocurrency deviates significantly from its recent average. Unlike centralized alert services, VolatileOracle leverages the transparency and immutability of blockchain technology to ensure verifiable and tamper-proof notifications. Users define their desired price thresholds and notification preferences within smart contracts, which continuously monitor price feeds from on-chain oracles. When a threshold is breached, the smart contract triggers a notification mechanism, alerting the user via various channels such as email or push notifications. This removes the reliance on trusted intermediaries and offers a more robust and auditable alert system.

The core design philosophy behind VolatileOracle is to minimize trust assumptions. By utilizing on-chain oracles for price data and smart contracts for alert logic, we eliminate the risk of data manipulation or service downtime inherent in centralized systems. The system is designed to be highly configurable, allowing users to customize alert thresholds, notification frequencies, and notification channels to suit their individual needs. Furthermore, the serverless architecture minimizes infrastructure costs and ensures continuous operation. The Rust implementation enhances security and performance critical for interacting with blockchain networks.

VolatileOracle empowers users to stay informed about market volatility in a secure, transparent, and reliable manner. It's ideal for traders, investors, and anyone who wants to be alerted to significant price movements in the cryptocurrency market without relying on potentially unreliable or opaque centralized services. The architecture is modular and extensible, allowing for the addition of new oracle integrations, notification channels, and alert types in the future. This project aims to foster greater trust and accessibility in the cryptocurrency space.

## Key Features

*   **Decentralized Alerts:** Triggers alerts based on conditions defined in smart contracts, eliminating reliance on centralized servers.
*   **On-Chain Oracles:** Utilizes on-chain price oracles (e.g., Chainlink, Band Protocol - currently configurable) for verifiable price data, ensuring data integrity. Configuration is handled via the `config.toml` file.
*   **Threshold-Based Triggers:** Allows users to define upper and lower price thresholds based on a rolling average, triggering alerts when these thresholds are crossed. The rolling average window is also configurable.
*   **Smart Contract Integration:** Employs smart contracts written in Solidity (example contracts provided in the `contracts/` directory) to manage user subscriptions, alert logic, and oracle data retrieval.
*   **Serverless Architecture:** Designed to run in a serverless environment, minimizing operational overhead and ensuring high availability.
*   **Configurable Notification Channels:** Supports multiple notification channels, including email (via SMTP) and push notifications (via a push notification service API - configurable in `config.toml`).
*   **Rust Implementation:** Written in Rust for enhanced security, performance, and reliability, leveraging Rust's memory safety features.

## Technology Stack

*   **Rust:** The primary programming language, providing memory safety and performance for interacting with blockchain networks.
*   **Solidity:** Used to write the smart contracts that define alert logic and manage subscriptions.
*   **Web3.rs:** A Rust library for interacting with Ethereum-compatible blockchains.
*   **Ethereum:** The blockchain platform where smart contracts are deployed and executed.
*   **Chainlink/Band Protocol (configurable):** On-chain oracle services providing real-time price data.
*   **SMTP (or equivalent email service):** For sending email notifications.
*   **TOML:** Configuration format for managing environment variables and settings.

## Installation

1.  **Install Rust:** Ensure you have Rust and Cargo installed. You can download them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2.  **Clone the Repository:**
    `git clone https://github.com/uhsr/VolatileOracle.git`
    `cd VolatileOracle`

3.  **Install Dependencies:**
    `cargo build`

4.  **Install Foundry (for deploying contracts, optional):** Instructions can be found at [https://book.getfoundry.sh/](https://book.getfoundry.sh/)

## Configuration

1.  **Create a `config.toml` file:** Copy the `config.example.toml` file to `config.toml` and modify the settings.

2.  **Environment Variables:**
    The following environment variables must be configured in the `config.toml` file:

    *   `ethereum_rpc_url`: The URL of your Ethereum node. (e.g., `"https://mainnet.infura.io/v3/<YOUR_INFURA_PROJECT_ID>"`)
    *   `private_key`: The private key of the Ethereum account that will interact with the smart contracts.
    *   `contract_address`: The address of the deployed VolatileOracle smart contract.
    *   `oracle_address`: The address of the price oracle contract.
    *   `smtp_server`: The SMTP server address for sending email notifications.
    *   `smtp_port`: The SMTP server port.
    *   `smtp_username`: The SMTP username.
    *   `smtp_password`: The SMTP password.
    *   `notification_email`: The email address to send notifications from.
    *   `recipient_email`: The email address to receive notifications.
    *   `price_deviation_threshold`: The percentage deviation from the rolling average that triggers an alert (e.g., `"0.05"` for 5%).
    *   `rolling_average_window`: The number of blocks to use for calculating the rolling average (e.g., `"10"`).
    *  `oracle_type`: Specifies the type of oracle being used (e.g., "Chainlink" or "Band").

3.  **Example `config.toml`:**
   (Note: This should be within a markdown code block, but removed to fulfill requirements)



## Usage

1.  **Compile the Smart Contracts (optional):**
    If you need to deploy or modify the smart contracts:
    `forge build` (requires Foundry)

2.  **Deploy the Smart Contracts (optional):**
    `forge create src/VolatileOracle.sol:VolatileOracle --rpc-url $ETHEREUM_RPC_URL --private-key $PRIVATE_KEY` (requires Foundry)

3.  **Run the VolatileOracle application:**
    `cargo run`

The application will connect to the Ethereum node, monitor the price oracle, and send notifications when the price deviation threshold is breached. The application logs will provide detailed information about the process, including price updates, threshold checks, and notification sending. To implement more complex functionalities, modify the Rust source code, particularly the `src/main.rs` file, which handles the core logic of the application.

## Contributing

We welcome contributions to VolatileOracle! Please follow these guidelines:

*   Fork the repository.
*   Create a new branch for your feature or bug fix.
*   Write clear and concise commit messages.
*   Submit a pull request with a detailed description of your changes.
*   Ensure your code adheres to the Rust style guidelines (using `cargo fmt`).
*   Include tests for your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/uhsr/VolatileOracle/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to thank the developers of Web3.rs, Chainlink, Band Protocol, and the Rust community for their contributions to the ecosystem.