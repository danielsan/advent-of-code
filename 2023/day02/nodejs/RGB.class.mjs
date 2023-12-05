export class RGB {
  red = 0
  green = 0
  blue = 0

  constructor (red = 0, green = 0, blue = 0) {
    this.red = red
    this.green = green
    this.blue = blue
  }

  hasAnyLargerThan (other) {
    return (
      this.red > other.red ||
      this.green > other.green ||
      this.blue > other.blue
    )
  }

  addValues (other) {
    this.red += other.red
    this.green += other.green
    this.blue += other.blue

    return this
  }

  equals (other) {
    return (
      this.red === other.red &&
      this.green === other.green &&
      this.blue === other.blue
    )
  }

  getPower () {
    return this.red * this.green * this.blue
  }

  static fromObject (obj) {
    return new this(obj.red, obj.green, obj.blue)
  }

  // String sample:
  // 11 red, 8 blue, 1 green
  static fromString (str) {
    const instance = new this()
    const list = str.split(/\s*,\s*/)

    for (const item of list) {
      const [value, color] = item.split(/\s+/)
      instance[color] = +value
    }

    return instance
  }
}

export default RGB
