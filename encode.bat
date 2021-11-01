@echo off

:: This will prevent errors is the full command call later
::set input=%1%
::set output=%2%
::set crf=%3%

:: This is printing out the given input and output file befor the main commands is prepared.
::echo Input File: %input%
::echo Output File: %output%
::echo CRF Rating: %crf%

::ffmpeg -hwaccel auto -i "%input%" -c:v libaom-av1 -crf %crf% -b:v 0 -pix_fmt yuv420p -row-mt 1 -tiles 4x4 -cpu-used 8 -c:a libopus -b:a 128K "%output%"

:: This is the arry ptr that increments to add to the array for each newly added file.
set qptr=0
set /A qptrmask=%qptr%-1

:PROGRAMSTART

:: This will iterate over the query and print out all the elements.
set /A qptrmask=%qptr%-1
if defined Query[0].input (
	for /l %%n in (0,1,%qptrmask%) do (
		call echo %%n: [INPUT: %%Query[%%n].input%%, OUTPUT: %%Query[%%n].output%%, CRF: %%Query[%%n].crf%%]
	)
) else echo Query is empty...

choice /c AEQ /m "Select Options: [A]dd file; [E]ncode Query; [Q]uit" /n
if errorlevel=3 goto END
if errorlevel=2 goto ENCODENOW
if errorlevel=1 goto ADDFILE

:ENCODENOW
set /A qptrmask=%qptr%-1
if defined Query[0].input (
	for /l %%n in (0,1,%qptrmask%) do (
		set ffmpeginput=Query[%%n].input
		set ffmpegoutput=Query[%%n].output
		set ffmpegcrf=Query[%%n].crf
		ffmpeg -hwaccel auto -i "%ffmpeginput%" -c:v libaom-av1 -crf %ffmpegcrf% -b:v 0 -pix_fmt yuv420p -row-mt 1 -tiles 4x4 -cpu-used 8 -c:a libopus -b:a 128K "%ffmpegoutput%"
		Rem cls
	)
) else echo Query is empty...

echo Encoding finished...
goto END

:: ADDFILE will prompt the user for input and output name, as well as crf rating, adding it to the array.
:ADDFILE

:: Gets the prompts and then adds them to the array
set /P input=Input File: 
set /P output=Output File: 
set /p crf=CRF Rating: 

set Query[%qptr%].input=%input%
set Query[%qptr%].output=%output%
set Query[%qptr%].crf=%crf%

:: Increment the ptr after!
set /A "qptr+=1"

goto PROGRAMSTART

:END
echo Program ended...
pause