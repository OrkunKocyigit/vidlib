import { Button, Center, Progress, Stack, Text } from '@mantine/core';
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
  const [name, setName] = useState<string | undefined>();
  const [progress, setProgress] = useState<number>(0);

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
      ({
        payload
      }: {
        payload: { current: number; total: number; progress: number; name?: string };
      }) => {
        setCurrent(payload.current);
        setTotal(payload.total);
        setProgress(payload.progress);
        setName(payload.name);
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
        <>
          {name !== undefined ? (
            <Text truncate={'end'} size={'sm'}>
              {name}
            </Text>
          ) : null}
          <Progress
            color="green"
            radius="md"
            size="xl"
            value={progress}
            label={`${current} / ${total}`}
            striped
            animate
          />
        </>
      ) : null}
      <FolderInput path={path} onSelect={setPath}></FolderInput>
      <Center>
        <Button onClick={scanFolder}>{t('add.folder.scan')}</Button>
      </Center>
    </Stack>
  );
}

export default AddFolderWizard;
