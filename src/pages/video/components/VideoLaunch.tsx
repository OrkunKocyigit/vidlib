import { type VideoFile } from '../../../entities/VideoFile';
import { ActionIcon, Group, Text } from '@mantine/core';
import { IconPlayerPlay } from '@tabler/icons-react';

export interface VideoLaunchProps {
  video: VideoFile;
}

function VideoLaunch(props: VideoLaunchProps): JSX.Element {
  return (
    <Group align={'center'}>
      <ActionIcon color={'blue'}>
        <IconPlayerPlay></IconPlayerPlay>
      </ActionIcon>
      <Text>Play</Text>
    </Group>
  );
}

export default VideoLaunch;
