import { queryPrice, batchQueryIndividualPrices } from '@shadeprotocol/shadejs';

// Function to fetch a single SHD price
window.fetchSHDPrice = async function () {
    try {
        const priceData = await queryPrice({
            contractAddress: "secret10n2xl5jmez6r9umtdrth78k0vwmce0l5m9f5dm",
            codeHash: "32c4710842b97a526c243a68511b15f58d6e72a388af38a7221ff3244c754e91",
            oracleKey: 'SHD',
        });
        
        const rateFormatted = (parseFloat(priceData.rate) / 1e18).toFixed(2);
        console.log("Formatted SHD Price:", rateFormatted);
        return rateFormatted;
    } catch (error) {
        console.error("Error fetching SHD price:", error);
    }
};

// Function to fetch multiple prices in a batch
window.fetchBatchPrices = async function () {
    try {
        const oracleKeys = ["SHD", "BTC", "ETH", "SCRT", "STKD-SCRT", "SILK"]; // Specify the tokens to query
        const priceData = await batchQueryIndividualPrices({
            queryRouterContractAddress: "secret15mkmad8ac036v4nrpcc7nk8wyr578egt077syt",
            queryRouterCodeHash: "1c7e86ba4fdb6760e70bf08a7df7f44b53eb0b23290e3e69ca96140810d4f432",
            oracleContractAddress: "secret10n2xl5jmez6r9umtdrth78k0vwmce0l5m9f5dm",
            oracleCodeHash: "32c4710842b97a526c243a68511b15f58d6e72a388af38a7221ff3244c754e91",
            oracleKeys: oracleKeys,
        });

        const formattedPrices = {};
        oracleKeys.forEach(key => {
            if (priceData[key]) {
                formattedPrices[key] = (parseFloat(priceData[key].rate) / 1e18).toFixed(2);
                console.log(`Formatted ${key} Price:`, formattedPrices[key]);
            } else {
                formattedPrices[key] = "Error";
                console.warn(`No price data found for ${key}`);
            }
        });
        return formattedPrices;
    } catch (error) {
        console.error("Error fetching batch prices:", error);
        return {};
    }
};
