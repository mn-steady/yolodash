import { queryPrice } from '@shadeprotocol/shadejs';

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
