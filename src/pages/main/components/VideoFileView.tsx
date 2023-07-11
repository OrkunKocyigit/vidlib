import React from "react"
import {VideoFile} from "../../../entities/VideoFile";

type Props = {
    video: VideoFile
}
function VideoFileView(props: Props) {
    return (
        <div>{props.video.path}</div>
    )
}

export default VideoFileView