import ReactDOM from "react-dom/client";
import App from "./App";
import { MantineProvider } from "@mantine/core";
import { NotificationsProvider } from "@mantine/notifications";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <MantineProvider withGlobalStyles withNormalizeCSS theme={{
        colorScheme: 'dark',
        colors: {
            yogurt: ['#ffe2e2', '#ffd8d8', '#ffcfcf', '#ffc6c6', '#ffbdbd', '#ffb4b4', '#ffabab', '#ffa2a2', '#ff9999','#ff9090'],
        },
        primaryColor: 'pink',
        globalStyles: (theme) => ({
            body: {
                userSelect: 'none'
            },
        }),
    }} >
        <NotificationsProvider>
            <App />
        </NotificationsProvider>
    </MantineProvider>
);
