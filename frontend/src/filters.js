const filters = {
    transformYoctoNear(yoctoNear) {
        return (yoctoNear / (10 ** 24)).toFixed(5)
    }
}
export default filters;
