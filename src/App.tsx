import "./App.css";
import {useEffect, useState} from "react";
import {ScanFile, ScanFileResult} from "./service/ScanFile";

function App() {
    let [result, setResult] = useState<ScanFileResult>()
    useEffect(() => {
        ScanFile("C:\\Projects\\vidlib\\test").then(response => {
            setResult(response.response)
        })
    }, [])
    return (
        <pre>
            { JSON.stringify(result, null, 2) }
        </pre>
    );
}

export default App;
