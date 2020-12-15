$installationPath = vswhere.exe -prerelease -latest -property installationPath
if ($installationPath -and (test-path "$installationPath\Common7\Tools\vsdevcmd.bat")) {
    & "${env:COMSPEC}" /s /c "`"$installationPath\Common7\Tools\vsdevcmd.bat`" -no_logo && set" | foreach-object {
        $name, $value = $_ -split '=', 2
        set-content env:\"$name" $value
    }
}

$commands = {
    msbuild.exe -t:restore
    msbuild.exe -t:build -p:Configuration=release
}
$encodedCommand = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($commands))
$pwsh = "pwsh.exe"
$cmdargs = """$installationPath\Common7\Tools\vsdevcmd.bat"" & $pwsh -e ""$encodedCommand"""

# & $env:comspec /k """$installationPath\Common7\Tools\vsdevcmd.bat"" & $pwsh -c ""echo 123;echo wqr;exit"" & exit"
Write-Host $cmdargs -ForegroundColor Green
& $env:comspec /c $cmdargs