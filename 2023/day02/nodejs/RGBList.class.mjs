import { RGB } from './RGB.class.mjs'

export class RGBList {
  sums = new RGB()
  list = []

  /**
   * Adds an RGB instance to the list
   * @param {RGB} rgb
   * @returns self
   */
  add (rgb) {
    if (!(rgb instanceof RGB)) {
      throw new TypeError('Expected instance of RGB')
    }

    this.list.push(rgb)
    this.sums.addValues(rgb)

    return this
  }

  getMaxPossibleRGB () {
    return new RGB(
      Math.max(...this.list.map(_ => _.red)),
      Math.max(...this.list.map(_ => _.green)),
      Math.max(...this.list.map(_ => _.blue))
    )
  }

  getPowerFrpmMaxPossibleRGB () {
    return this.getMaxPossibleRGB().getPower()
  }

  static fromString (str) {
    return str.split(/\s*;\s*/).reduce((list, item) => {
      list.add(RGB.fromString(item))

      return list
    }, new this())
  }
}

export default RGBList
