# 关于这个软件
你可以通过配置json文件来方便的打开你配置的软件或文件  
# 如何配置
同一开头的变量的一组数据组成一次完整的事件例如`a3_1 a3_2 a3_3 a3_name a3_path`的第一位数据组成一次事件  
a3开头的变量为按3次按钮打开软件  
b4开头的变量为按4次按钮打开软件  
b5开头的变量为按5次按钮打开软件
d开头的变量为按6按键次打开文件(只有d开头的变量可以打开文件也只可以打开文件)  

_1 _2 _3 _4 _5 分别代表第一,第二,第三,第四,第五次输入  
例如在A3_1中输入"up"在A3_2中输入"Right"在A3_3中输入"Right"(不区分大小写),此时打开软件在方向键上依次按下 上右左 就可以打开指定的软件  
d_file1,2,3,4,5,6代表第1,2,3,4,5,6次输入  

_path 代表需要打开的文件或路径  
路径格式为 `盘符:\\文件夹\\软件名称`

_name 代表你对软件或文件的称呼请尽量让它更容易被你读懂  
# 注意  
输入次数大的事件中的输入顺序不得包含输入次数小的事件的输入顺序   
可以在变量中添加,来增加事件  
例: A3_1为up A3_2为up A3_3为up 就不可以输入 B4_1为up B4_2为up B4_3为up B4_4为down 否则只会执行A3的事件而不执行B4的事件  
如果提示请求的操作需要提升请使用管理员权限运行软件  
如果输出的emoji符号异常可以使用Segoe UI Emoji字体

# About
You can easily open the software or file you configured by configuring the json file  
# How to configure
A set of data from a variable with the same beginning forms a complete event e.g. the first bit of data from `a3_1 a3_2 a3_3 a3_name a3_path` forms an event.  
Variables beginning with A3 open the software by pressing the button 3 times.  
Variables beginning with B4 open the software by pressing the button 4 times.  
Variables beginning with C5 open the software by pressing the button 5 times.  
Variables beginning with d open the file by pressing 6 keystrokes (only variables beginning with d can open the file and only the file can be opened)  

_1 _2 _3 _4 _5 represent the first, second, third, fourth and fifth inputs respectively.  
For example, in A3_1 enter “up” in A3_2 enter “Right” in A3_3 enter “Right” (case insensitive), this time to open the software in the direction of the key in turn press on the right left to open the specified software

_path on behalf of the need to open the file or path  
The path format is `C:\\folder\\qwe.exe`  
_name is the name of the software or file you're referring to. Please try to make it as easy to read as possible.  

# Note  
The input order of events with a large number of inputs must not include the input order of events with a small number of inputs.  
You can add a variable to increase the number of events.  
Example: A3_1 is up A3_2 is up A3_3 is up, then B4_1 is up B4_2 is up B4_3 is up B4_4 is down, otherwise it will only execute the event of A3 but not the event of B4.  
If you are prompted that 请求的操作需要提升, please run the software with administrator privileges.  
If the output emoji symbols are abnormal you can use the Segoe UI Emoji font  

Translated with DeepL.com (free version)
