import { type VideoFile } from '../../entities/VideoFile';

export interface VideoViewProps {
  video?: VideoFile;
}

const VideoView = function (props: VideoViewProps): JSX.Element {
  return <div>{JSON.stringify(props.video, null, 4)}</div>;
};

export default VideoView;
