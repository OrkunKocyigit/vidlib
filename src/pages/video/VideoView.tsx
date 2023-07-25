import { type VideoFile } from '../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { GetVideo } from '../../service/GetVideo';
import { type VideoEntry } from '../../entities/VideoEntry';
import { GetThumbnail } from '../../service/GetThumbnail';
import { Box } from '@mantine/core';

export interface VideoViewProps {
  video?: VideoFile;
}

const VideoView = function (props: VideoViewProps): JSX.Element {
  const [videoEntry, setVideoEntry] = useState<VideoEntry | undefined>(undefined);
  const [imageSrc, setImageSrc] = useState<string[] | undefined>(undefined);
  useEffect(() => {
    if (props.video != null) {
      GetVideo(props.video)
        .then((value) => {
          setVideoEntry(value.response as VideoEntry);
        })
        .catch((reason) => {
          console.error(reason);
        });
      GetThumbnail(props.video)
        .then((value) => {
          setImageSrc(value.response);
        })
        .catch((reason) => {
          console.error(reason);
        });
    }
  }, [props.video]);
  return (
    <Box>
      {JSON.stringify(videoEntry)}
      {JSON.stringify(imageSrc)}
    </Box>
  );
};

export default VideoView;
