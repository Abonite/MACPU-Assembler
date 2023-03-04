# pycpu_assembler

A assembler of my MACPU

---

## About

In order to drive MACPU and make it have more application scenarios, we must implement a compiler of MACPU assembly language. In order to implement this compiler, I chose the high-performance and memory-safe rust language for development.

For the algorithm model of MACPU, please refer to [here](https://github.com/Abonite/MACPU-model); for the FPGA implementation of MACPU, please refer to [here](https://github.com/Abonite/MACPU-FPGA).

At present, this assembler only supports single-file compilation, that is, it does not have the link function for the time being.

---

工作原理

1. 读取文件并将文件按行拆分，获得一个Vector，其中的每一个元素都是行字符串与行号的元组
2. 遍历所有以点号开头的行，并将其按照预处理指令的要求解析（DEFINE，DATA，ARRAY，SET），将有效的指令从Vector中删除，创建一个协程执行需要进行字符串替换的预处理指令（DEFINE，DATA，ARRAY），使用正则表达式。要求DEFINE指令被先处理，DATA次之，ARRAY再次。三种指令不能有重复的命名，不能以数字或引号开头，要对规则进行检查。处理DATA、ARRAY时，局应当同时生成数据端数组和记录变量的hashmap。在进行过DEFINE替换后，再将代码中的非DEFINE变量用hashmap替换
3. 获取SET指令的设置，并以此为基准进行后续操作
4. 按行读取指令，先获取所有的有效行数，并创建一个数组，每一个元素都是语法分析树。每读取一行就再给行标记一个连续的新标号，再以协程进行词法分析，词法分析应使用状态机逐个字符处理，拼接语法分析树，协程结束后按标号将语法分析树写入数组中。协程的返回值是result，将所有异常单独收集并统一列出
5. 如果所有协程都没有异常，则再开始读取语法分析树生成二进制数据，将它们写入预估好大小的数组，并记录标签的位置，所有涉及标签的指令都应该单独最后执行
