import { Flex, Group, Text, Textarea, Title } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import { useTranslation } from 'react-i18next';
import EditableButtons from '../../../components/EditableButtons';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';

export interface VideoNotesProps {
  video: VideoFile;
}
function VideoNotes(props: VideoNotesProps): JSX.Element {
  const { t } = useTranslation();
  const [editable, { open, close }] = useDisclosure(false);
  const [notes, setNotes] = useState(props.video.video?.notes);

  useEffect(() => {
    close();
    setNotes(props.video.video?.notes);
  }, [props.video.video?.notes]);

  function onSaveNotes(): void {
    close();
  }

  function onCancelNotes(): void {
    close();
  }

  return (
    <Flex direction={'column'} gap={'md'} h={'100%'}>
      <Group>
        <Title>{t('video.notes.title')}</Title>
        <EditableButtons
          editable={editable}
          onEditButtonClick={open}
          editButtonProps={{ color: 'blue' }}
          onSaveButtonClick={onSaveNotes}
          saveButtonProps={{ color: 'green' }}
          onCancelButtonClick={onCancelNotes}
          cancelButtonProps={{ color: 'red' }}></EditableButtons>
      </Group>
      {editable ? (
        <Textarea
          value={notes}
          radius={'md'}
          variant={'filled'}
          minRows={4}
          autosize
          onChange={(e) => {
            setNotes(e.target.value);
          }}></Textarea>
      ) : (
        <Text>{props.video.video?.notes}</Text>
      )}
    </Flex>
  );
}

export default VideoNotes;
