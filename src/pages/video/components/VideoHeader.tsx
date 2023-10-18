import { Flex, Group, Text } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import VideoThumbnail from './VideoThumbnail';
import VideoRating from './VideoRating';
import VideoWatch from './VideoWatch';
import VideoLaunch from './VideoLaunch';
import VideoName from './VideoName';

export interface VideoHeaderProps {
  video: VideoFile;
}

function VideoHeader(props: VideoHeaderProps): JSX.Element {
  return (
    <Group wrap="nowrap" align="start">
      <VideoThumbnail video={props.video}></VideoThumbnail>
      <Flex direction="column" gap="md">
        <VideoName video={props.video}></VideoName>
        <Text>{props.video.path}</Text>
        <Flex gap="lg">
          <VideoRating video={props.video}></VideoRating>
          <VideoWatch video={props.video}></VideoWatch>
          <VideoLaunch video={props.video}></VideoLaunch>
        </Flex>
      </Flex>
    </Group>
  );
}

export default VideoHeader;
