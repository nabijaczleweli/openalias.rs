openalias.rs(1) -- Securely look up and parse OpenAlias data
=========================================================

## SYNOPSIS

`openalias` &lt;OPEN_ALIAS&gt;... [OPTIONS]

## DESCRIPTION

Securely look up and parse OpenAlias data.

OpenAlias is an open DNS-based name to cryptocurrency address mapping format.

`openalias.rs` utilises DNSCrypt and official OpenAlias DNS servers
in order to ensure that your lookups are safe.

## OPTIONS

  &lt;OPEN_ALIAS&gt;...

    FQDN or email-style aliases to look up addresses for.

  -v --verbose

    Print more data about what's happenning to stderr.

    Default: don't.

## EXAMPLES

  `openalias nabijaczleweli.xyz donate.getmonero.org`

    Addresses of nabijaczleweli.xyz:
      btc:
        nabijaczleweli
        1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH

    Addresses of donate.getmonero.org:
      xmr:
        Monero Development
        44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A
        Donation to Monero Core Team
      btc:
        Monero Development
        1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H
        Donation to Monero Core Team

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/openalias.rs/issues>&gt;

## SEE ALSO

&lt;<https://openalias.org>&gt;<br />
&lt;<https://github.com/nabijaczleweli/openalias.rs>&gt;
