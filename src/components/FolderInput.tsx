import { Button } from '@mantine/core';
import { useState } from 'react';
import { SelectFolder } from '../service/SelectFolder';

export interface FolderInputProps {
  path?: string;
  onSelect: (path: string) => void;
}
function FolderInput(props: FolderInputProps): JSX.Element {
  const [text, setText] = useState<string>('Select Folder');

  const selectFolder = (): void => {
    SelectFolder()
      .then((value) => {
        setText(value.response as string);
        props.onSelect(value.response as string);
      })
      .catch((error) => {
        console.error(error);
      });
  };

  return (
    <Button variant="default" color="dark" radius="md" w="100%" onClick={selectFolder}>
      {text}
    </Button>
  );
}

export default FolderInput;
