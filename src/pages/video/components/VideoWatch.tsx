import { type VideoFile } from '../../../entities/VideoFile';
import { useState } from 'react';
import { ActionIcon, Box, Group, Text } from '@mantine/core';
import { IconCheck, IconX } from '@tabler/icons-react';

interface VideoWatchProps {
  video: VideoFile;
}

function VideoWatch(props: VideoWatchProps): JSX.Element {
  const [watched, setWatched] = useState(props.video?.video?.watched);

  function updateWatched(video: VideoFile, value: boolean): void {
    setWatched(value);
  }

  return (
    <Box>
      {watched === true ? (
        <Group
          align={'center'}
          onClick={() => {
            updateWatched(props.video, false);
          }}>
          <ActionIcon color={'green'}>
            <IconCheck></IconCheck>
          </ActionIcon>
          <Text>Watched</Text>
        </Group>
      ) : (
        <Group
          onClick={() => {
            updateWatched(props.video, true);
          }}>
          <ActionIcon color={'red'}>
            <IconX></IconX>
          </ActionIcon>
          <Text>Not Watched</Text>
        </Group>
      )}
    </Box>
  );
}

export default VideoWatch;
