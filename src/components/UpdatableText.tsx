import {
  ActionIcon,
  type ActionIconProps,
  Grid,
  Group,
  TextInput,
  type TextInputProps,
  Title,
  type TitleProps
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IconCheck, IconEdit, IconX } from '@tabler/icons-react';
import { useEffect, useState } from 'react';

export interface UpdatableTextInnerProps {
  text: string;
  setText: (t: string) => void;
  titleProps?: TitleProps;
  editButtonProps?: ActionIconProps;
  saveButtonProps?: ActionIconProps;
  cancelButtonProps?: ActionIconProps;
  textInputProps?: TextInputProps;
}

function UpdatableText(props: UpdatableTextInnerProps): JSX.Element {
  const [newText, setNewText] = useState(props.text);
  const [editable, { open, close }] = useDisclosure(false);

  useEffect(() => {
    close();
    setNewText(props.text);
  }, [props.text]);

  function onSaveText(): void {
    props.setText(newText);
    close();
  }

  function onCancelText(): void {
    setNewText(props.text);
    close();
  }

  return (
    <Grid>
      <Grid.Col span={9}>
        {editable ? (
          <TextInput
            {...props.textInputProps}
            value={newText}
            onChange={(e) => {
              setNewText(e.target.value);
            }}></TextInput>
        ) : (
          <Title {...props.titleProps}>{props.text}</Title>
        )}
      </Grid.Col>
      <Grid.Col span={3}>
        {editable ? (
          <Group>
            <ActionIcon {...props.saveButtonProps} onClick={onSaveText}>
              <IconCheck></IconCheck>
            </ActionIcon>
            <ActionIcon {...props.cancelButtonProps} onClick={onCancelText}>
              <IconX></IconX>
            </ActionIcon>
          </Group>
        ) : (
          <Group>
            <ActionIcon {...props.editButtonProps} onClick={open}>
              <IconEdit></IconEdit>
            </ActionIcon>
          </Group>
        )}
      </Grid.Col>
    </Grid>
  );
}

export default UpdatableText;
