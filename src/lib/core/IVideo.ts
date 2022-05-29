export default interface Video {
  title: String,
  description: String,
  thumbnail: String,
  duration: number, // Duration in seconds.
  channel: {
    name: String,
    thumbnail: String,
  }
}