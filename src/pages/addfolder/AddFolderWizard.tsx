import { Button, Center, Stack } from '@mantine/core';
import React, { useState } from 'react';
import FolderInput from '../../components/FolderInput';
import { type FolderInfo } from '../../entities/FolderInfo';
import { AddFolder } from '../../service/AddFolder';

export interface AddFolderWizardProps {
  onFolderAdd: (folderInfo: FolderInfo) => void;
}

function AddFolderWizard(props: AddFolderWizardProps): JSX.Element {
  const [path, setPath] = useState<string>('');

  function scanFolder(): void {
    AddFolder(path)
      .then((value) => {
        props.onFolderAdd(value.response as FolderInfo);
      })
      .catch((error) => {
        console.error(error);
      });
  }

  return (
    <Stack>
      <FolderInput path={path} onSelect={setPath}></FolderInput>
      <Center>
        <Button onClick={scanFolder}>Scan</Button>
      </Center>
    </Stack>
  );
}

export default AddFolderWizard;
