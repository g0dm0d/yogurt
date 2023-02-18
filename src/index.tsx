import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ColorSchemeProvider, GlobalStyles, MantineProvider, MantineThemeOverride } from "@mantine/core";
import { NotificationsProvider } from "@mantine/notifications";
import { useColorScheme, useLocalStorage } from "@mantine/hooks";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <MantineProvider theme={{ colorScheme: 'dark' }} withGlobalStyles withNormalizeCSS>
        <NotificationsProvider>
            <App />
        </NotificationsProvider>
    </MantineProvider>
);
