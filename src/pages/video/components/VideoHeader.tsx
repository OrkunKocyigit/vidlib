import { Flex, Group, Text, Title } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import VideoThumbnail from './VideoThumbnail';
import VideoRating from './VideoRating';
import VideoWatch from './VideoWatch';
import VideoLaunch from './VideoLaunch';

export interface VideoHeaderProps {
  video: VideoFile;
}

function VideoHeader(props: VideoHeaderProps): JSX.Element {
  return (
    <Group noWrap align={'start'}>
      <VideoThumbnail video={props.video}></VideoThumbnail>
      <Flex direction={'column'} gap={'md'}>
        <Title order={4}>{props.video?.name}</Title>
        <Text>{props.video?.path}</Text>
        <Flex gap={'lg'}>
          <VideoRating video={props.video}></VideoRating>
          <VideoWatch video={props.video}></VideoWatch>
          <VideoLaunch video={props.video}></VideoLaunch>
        </Flex>
      </Flex>
    </Group>
  );
}

export default VideoHeader;
