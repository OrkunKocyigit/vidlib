import { Button } from '@mantine/core';
import { useState } from 'react';
import { SelectFolder } from '../service/SelectFolder';
import { useTranslation } from 'react-i18next';

export interface FolderInputProps {
  path?: string;
  onSelect: (path: string) => void;
}
function FolderInput(props: FolderInputProps): JSX.Element {
  const { t } = useTranslation();
  const [text, setText] = useState<string>(t('folderinput.select'));

  const selectFolder = (): void => {
    SelectFolder()
      .then((value) => {
        const str = value.response ?? '';
        setText(str);
        props.onSelect(str);
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
