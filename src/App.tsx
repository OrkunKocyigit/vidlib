import './App.css';
import React, { useState } from 'react';
import MainView from './pages/main/MainView';
import { type ColorScheme, ColorSchemeProvider, MantineProvider } from '@mantine/core';

function App(): JSX.Element {
  const [colorScheme, setColorScheme] = useState<ColorScheme>('light');
  const toggleColorScheme = (value?: ColorScheme): void => {
    setColorScheme(value ?? (colorScheme === 'dark' ? 'light' : 'dark'));
  };

  return (
    <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
      <MantineProvider withGlobalStyles withNormalizeCSS>
        <MainView></MainView>
      </MantineProvider>
    </ColorSchemeProvider>
  );
}

export default App;
