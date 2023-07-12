import React from "react"
import {VideoFile} from "../../../entities/VideoFile";

type Props = {
    video: VideoFile
}
function VideoFileView(props: Props) {
    return (
        <div>{props.video.name}</div>
    )
}

export default VideoFileView