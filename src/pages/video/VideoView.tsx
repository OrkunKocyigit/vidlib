import { type VideoFile } from '../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { GetVideo } from '../../service/GetVideo';
import { type VideoEntry } from '../../entities/VideoEntry';
import { GetThumbnail } from '../../service/GetThumbnail';
import { Avatar, Flex, Group } from '@mantine/core';

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
    <Flex>
      <Group noWrap>
        {imageSrc != null ? <Avatar size={'xl'} radius={'md'} src={imageSrc[0]}></Avatar> : null}
      </Group>
      {JSON.stringify(videoEntry)}
      {JSON.stringify(imageSrc)}
    </Flex>
  );
};

export default VideoView;
