import './App.css'
import {Button, Flex, Layout, message, Spin, Splitter} from "antd"
import {Content, Header, Footer} from "antd/es/layout/layout"
import TextArea from "antd/es/input/TextArea"
import {CopyOutlined, InboxOutlined} from "@ant-design/icons"
import {Typography} from 'antd'
import Dragger from "antd/es/upload/Dragger"
import {useState} from "react";
import type { UploadRequestOption } from 'rc-upload/lib/interface';
import {merge} from "@ziyuecommentary/character_collector_wasm";

const {Title} = Typography

function App() {
    initWasm();
    const [messageApi, contextHolder] = message.useMessage();
    const [processing, setProcessing] = useState(false);

    const copyToClipboard = async () => {
        const outputElement = document.getElementById("output") as HTMLTextAreaElement;
        if (outputElement) {
            try {
                await navigator.clipboard.writeText(outputElement.value);
                messageApi.success("Copied!");
            } catch (err) {
                messageApi.error("Error when copying: " + err);
            }
        }
    };

    const handleFileUpload = (options: UploadRequestOption) => {
        const { file, onSuccess, onError, onProgress } = options;
        setProcessing(true);
        console.dir(file);
        const outputElement = document.getElementById("output") as HTMLTextAreaElement;
        merge(outputElement.value, "asd");
        onSuccess?.("ok", file);
        setProcessing(false);
    }

    return (
        <>
            {contextHolder}
            <Layout id={"layout"}>
                <Header id={"header"}>
                    <div className={"logo"}>Character Collector</div>
                </Header>
                <Content id={"content"}>
                    <Splitter>
                        <Splitter.Panel>
                            <Flex vertical align={"center"} style={{padding: '10px'}} gap={"large"}>
                                <Title level={2}>Input</Title>
                                <Dragger multiple customRequest={handleFileUpload}>
                                    <p className="ant-upload-drag-icon"><InboxOutlined/></p>
                                    <p className="ant-upload-text">Click or drag file to this area to collect</p>
                                    <p className="ant-upload-hint">Collect unique characters in files. Support for a
                                        single or bulk. The process runs on
                                        your computer, so you can collect characters safely.</p>
                                </Dragger>
                            </Flex>
                        </Splitter.Panel>
                        <Splitter.Panel>
                            <Spin tip="Processing..." spinning={processing}>
                                <Flex vertical align={"center"} style={{padding: '10px'}} gap={"large"}>
                                    <Title level={2}>Result</Title>
                                    <TextArea id={"output"}></TextArea>
                                    <Button type={"primary"} icon={<CopyOutlined/>} size={"large"}
                                            style={{width: '100%'}} onClick={copyToClipboard}>Copy</Button>
                                </Flex>
                            </Spin>
                        </Splitter.Panel>
                    </Splitter>
                </Content>
                <Footer id={"footer"}>
                    ZiYueCommentary 2025
                </Footer>
            </Layout>
        </>
    )
}

export default App