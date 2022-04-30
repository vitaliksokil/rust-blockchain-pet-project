const filters = {
    transformYoctoNear(yoctoNear) {
        return yoctoNear / (10 ** 24)
    }
}
export default filters;
