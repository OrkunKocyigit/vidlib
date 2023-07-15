import { type VideoFile } from '../../entities/VideoFile';

export interface VideoViewProps {
  video?: VideoFile;
}

const VideoView = function (props: VideoViewProps): JSX.Element {
  return <div>{props.video?.displayName}</div>;
};

export default VideoView;
