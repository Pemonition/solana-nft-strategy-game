import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { WalletContext } from "./WalletContext";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <WalletContext>
        <App />
    </WalletContext>
);
