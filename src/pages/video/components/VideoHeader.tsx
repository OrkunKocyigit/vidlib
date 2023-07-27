import { Flex, Group, Text, Title } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import VideoThumbnail from './VideoThumbnail';
import VideoRating from './VideoRating';

export interface VideoHeaderProps {
  video?: VideoFile;
}

function VideoHeader(props: VideoHeaderProps): JSX.Element {
  return (
    <Group noWrap align={'start'}>
      <VideoThumbnail video={props.video}></VideoThumbnail>
      <Flex direction={'column'} gap={'md'}>
        <Title order={4}>{props.video?.name}</Title>
        <Text>{props.video?.path}</Text>
        <Group>
          <VideoRating video={props.video}></VideoRating>
        </Group>
      </Flex>
    </Group>
  );
}

export default VideoHeader;
