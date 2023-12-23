import './App.css';
import '@mantine/core/styles.css';
import 'mantine-contextmenu/styles.css';
import React from 'react';
import MainView from './pages/main/MainView';
import { ColorSchemeScript, createTheme, MantineProvider } from '@mantine/core';
import { ContextMenuProvider } from 'mantine-contextmenu';

const theme = createTheme({
  primaryColor: 'red'
});

function App(): JSX.Element {
  return (
    <>
      <ColorSchemeScript defaultColorScheme="auto"></ColorSchemeScript>
      <MantineProvider theme={theme} defaultColorScheme="light">
        <ContextMenuProvider>
          <MainView></MainView>
        </ContextMenuProvider>
      </MantineProvider>
    </>
  );
}

export default App;
