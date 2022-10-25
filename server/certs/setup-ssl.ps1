# https://blog.devolutions.net/2020/07/tutorial-how-to-generate-secure-self-signed-server-and-client-certificates-with-openssl/
# https://security.stackexchange.com/a/166645
param
(
    [Switch]
    $Clean
)
$openssl = 'C:\Program Files\OpenSSL-Win64\bin\openssl.exe'

$Files = @{
    # Root CA Private Key
    'ca.key'     = @{ Index = 0; Args = 'ecparam -name prime256v1 -genkey -noout -out ca.key' }
    # Root CA Public Key
    'ca.crt'     = @{ Index = 1; Args = 'req -new -x509 -sha256 -key ca.key -out ca.crt -config ca.cfg -utf8' }
    # Server Private Key
    'server.key' = @{ Index = 2; Args = 'ecparam -name prime256v1 -genkey -noout -out server.key' }
    # Server Certificate Signing Request
    'server.csr' = @{ Index=  3; Args = 'req -new -sha256 -key server.key -out server.csr -config server.cfg -utf8' }
    # Sign Public Key signed with Root CA
    'server.crt' = @{ Index = 4; Args = 'x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 1000 -sha256 -extensions v3_req -extfile server.cfg' }
}

$Location = Get-Location
try
{
    Set-Location -Path $PSScriptRoot
    @('ca.cfg', 'server.cfg') | ForEach-Object -Process {
        if (-not (Test-Path -Path $PSItem -PathType Leaf))
        {
            Write-Error -Message "Missing file $PSItem, copy the example $PSItem.example and rename it accordingly"
        }
    }

    if ($Clean)
    {
        Get-Item -Path '.' -Include $Files.Keys | Remove-Item
    }

    $Files.GetEnumerator() | Sort-Object -Property {$_.Value.Index} | ForEach-Object -Process {
        $File = $PSItem.Name
        $Command = $PSItem.Value.Args
        if (-not (Test-Path -Path $File -PathType Leaf))
        {
            $OpenSSLArgs = $Command.Split(' ')
            & $openssl $OpenSSLArgs
        }
    }
}
finally
{
    Set-Location -Path $Location
}
