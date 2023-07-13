import { Button, Center, Stack } from '@mantine/core';
import React, { useState } from 'react';
import FolderInput from '../../components/FolderInput';
import { ScanFile } from '../../service/ScanFile';

function AddFolderWizard(): JSX.Element {
  const [path, setPath] = useState<string>('');

  function scanFolder(): void {
    ScanFile(path)
      .then((value) => {
        console.log(value);
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
