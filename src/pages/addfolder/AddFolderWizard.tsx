import { Button, Center, Progress, Stack } from '@mantine/core';
import React, { useEffect, useState } from 'react';
import FolderInput from '../../components/FolderInput';
import { type FolderInfo } from '../../entities/FolderInfo';
import { AddFolder } from '../../service/AddFolder';
import { useTranslation } from 'react-i18next';
import { listen } from '@tauri-apps/api/event';

export interface AddFolderWizardProps {
  onFolderAdd: (folderInfo: FolderInfo) => void;
}

function AddFolderWizard(props: AddFolderWizardProps): JSX.Element {
  const [path, setPath] = useState<string>('');
  const { t } = useTranslation();
  const [current, setCurrent] = useState<number>(0);
  const [total, setTotal] = useState<number>(0);
  const [name, setName] = useState<string>('');

  function scanFolder(): void {
    setCurrent(0);
    setTotal(0);
    setName('');
    AddFolder(path)
      .then((value) => {
        props.onFolderAdd(value.response as FolderInfo);
      })
      .catch((error) => {
        console.error(error);
      });
  }

  useEffect(() => {
    const unlisten = listen(
      'add_progress',
      ({ payload }: { payload: { total?: number; name?: string } }) => {
        setTotal((prevState) => prevState + (payload.total ?? 0));
        if (payload.name !== undefined) {
          setName(payload.name);
          setCurrent((prevState) => prevState + 1);
        }
      }
    );
    return () => {
      unlisten
        .then((value) => {
          value();
        })
        .catch((reason) => {
          console.error(reason);
        });
    };
  }, []);

  return (
    <Stack>
      {total > 0 ? (
        <Progress
          color="green"
          radius="md"
          size="xl"
          value={Math.min(1, Math.max(0, current / total)) * 100}
          label={name}
          striped
          animate
        />
      ) : null}
      <FolderInput path={path} onSelect={setPath}></FolderInput>
      <Center>
        <Button onClick={scanFolder}>{t('add.folder.scan')}</Button>
      </Center>
    </Stack>
  );
}

export default AddFolderWizard;
