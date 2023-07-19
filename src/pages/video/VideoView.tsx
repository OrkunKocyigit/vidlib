import { type VideoFile } from '../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { GetVideo } from '../../service/GetVideo';
import { type VideoEntry } from '../../entities/VideoEntry';

export interface VideoViewProps {
  video?: VideoFile;
}

const VideoView = function (props: VideoViewProps): JSX.Element {
  const [videoEntry, setVideoEntry] = useState<VideoEntry | null>(null);
  useEffect(() => {
    if (props.video != null) {
      GetVideo(props.video)
        .then((value) => {
          setVideoEntry(value.response as VideoEntry);
        })
        .catch((reason) => {
          console.error(reason);
        });
    }
  }, [props.video]);
  return <div>{JSON.stringify(videoEntry, null, 4)}</div>;
};

export default VideoView;
