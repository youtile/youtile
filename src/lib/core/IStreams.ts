export default interface Streams {
    video: VideoStream[],
    audio: AudioStream[],
}

interface VideoStream {
    url: string,
    resolution: {
        width: number,
        height: number,
        fps: number,
        mineType: string,
    }
}

interface AudioStream {
    url: string,
    quality: {
        audioSampleRate: number,
        audioChannels: number,
    }
}