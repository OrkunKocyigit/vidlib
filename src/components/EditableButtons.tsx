import { ActionIcon, type ActionIconProps, Group, useComponentDefaultProps } from '@mantine/core';
import { IconCheck, IconEdit, IconX } from '@tabler/icons-react';

export interface EditableButtonsProps {
  editable: boolean;
  saveButtonProps?: ActionIconProps;
  onSaveButtonClick?: () => void;
  cancelButtonProps?: ActionIconProps;
  onCancelButtonClick?: () => void;
  editButtonProps?: ActionIconProps;
  onEditButtonClick?: () => void;
}

const defaultProps: Partial<EditableButtonsProps> = {
  onSaveButtonClick: () => {},
  onCancelButtonClick: () => {},
  onEditButtonClick: () => {}
};

function EditableButtons(props: EditableButtonsProps): JSX.Element {
  const {
    editable,
    cancelButtonProps,
    onCancelButtonClick,
    saveButtonProps,
    onSaveButtonClick,
    editButtonProps,
    onEditButtonClick
  } = useComponentDefaultProps('EditableButtons', defaultProps, props);
  return (
    <>
      {editable ? (
        <Group>
          <ActionIcon {...saveButtonProps} onClick={onSaveButtonClick}>
            <IconCheck></IconCheck>
          </ActionIcon>
          <ActionIcon {...cancelButtonProps} onClick={onCancelButtonClick}>
            <IconX></IconX>
          </ActionIcon>
        </Group>
      ) : (
        <Group>
          <ActionIcon {...editButtonProps} onClick={onEditButtonClick}>
            <IconEdit></IconEdit>
          </ActionIcon>
        </Group>
      )}
    </>
  );
}

export default EditableButtons;
