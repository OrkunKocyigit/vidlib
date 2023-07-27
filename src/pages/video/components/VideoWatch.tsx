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
        <Group align={'center'}>
          <ActionIcon
            color={'green'}
            onClick={() => {
              updateWatched(props.video, false);
            }}>
            <IconCheck></IconCheck>
          </ActionIcon>
          <Text>Watched</Text>
        </Group>
      ) : (
        <Group>
          <ActionIcon
            color={'red'}
            onClick={() => {
              updateWatched(props.video, true);
            }}>
            <IconX></IconX>
          </ActionIcon>
          <Text>Not Watched</Text>
        </Group>
      )}
    </Box>
  );
}

export default VideoWatch;
