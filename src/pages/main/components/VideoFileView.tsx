import React from 'react';
import { type VideoFile } from '../../../entities/VideoFile';

interface Props {
  video: VideoFile;
}
function VideoFileView(props: Props): JSX.Element {
  return <div>{props.video.name}</div>;
}

export default VideoFileView;
