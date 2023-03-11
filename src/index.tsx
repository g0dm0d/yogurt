import ReactDOM from "react-dom/client";
import App from "./App";
import { MantineProvider } from "@mantine/core";
import { NotificationsProvider } from "@mantine/notifications";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <MantineProvider withGlobalStyles withNormalizeCSS theme={{
        colorScheme: 'dark',
        colors: {
            yogurt: ['#F0BBDD', '#ED9BCF', '#EC7CC3', '#ED5DB8', '#F13EAF', '#F71FA7', '#FF00A1', '#E00890', '#C50E82', '#AD1374'],
        },
        primaryColor: 'yogurt',
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
