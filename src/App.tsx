import "./App.css";
import React, {useState} from "react";
import MainView from "./pages/main/MainView";
import {ColorScheme, ColorSchemeProvider, MantineProvider} from "@mantine/core";

function App() {
    const [colorScheme, setColorScheme] = useState<ColorScheme>('light');
    const toggleColorScheme = (value?: ColorScheme) =>
        setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));

    return (
        <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
            <MantineProvider withGlobalStyles withNormalizeCSS>
                <MainView></MainView>
            </MantineProvider>
        </ColorSchemeProvider>
    );
}

export default App;
