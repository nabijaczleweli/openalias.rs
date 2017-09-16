openalias.rs(1) -- Look up and parse OpenAlias data
===================================================

## SYNOPSIS

`openalias` &lt;OPEN_ALIAS&gt;... [OPTIONS]

## DESCRIPTION

Look up and parse OpenAlias data.

OpenAlias is an open DNS-based name to cryptocurrency address mapping format.

## OPTIONS

  &lt;OPEN_ALIAS&gt;...

    FQDN or email-style aliases to look up addresses for.

  -v --verbose

    Print more data about what's happenning to stderr.

    Default: don't.

  -r --raw

    Print just the record text.

    Default: pretty-printing.

## EXAMPLES

  `openalias nabijaczleweli.xyz donate.getmonero.org`

    Addresses of nabijaczleweli.xyz:
      btc:
        nabijaczleweli
        1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH

    Addresses of donate.getmonero.org:
      xmr:
        Monero Development
        44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3
          XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A
        Donation to Monero Core Team
      btc:
        Monero Development
        1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H
        Donation to Monero Core Team

  `openalias -rv nabijaczleweli.xyz donate@getmonero.org`

    Looking up nabijaczleweli.xyz...
    Addresses for nabijaczleweli.xyz:
      oa1:btc recipient_address=1CgLs6CxXMAY4Pj4edQq5vyaFoP9NdqVKH;
              recipient_name=nabijaczleweli;

    Looking up donate@getmonero.org...
    Addresses for donate@getmonero.org:
      oa1:xmr recipient_address=44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3
                                  XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A;
              recipient_name=Monero Development;
              tx_description=Donation to Monero Core Team;
      oa1:btc recipient_address=1KTexdemPdxSBcG55heUuTjDRYqbC5ZL8H;
              recipient_name=Monero Development;
              tx_description=Donation to Monero Core Team;

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/openalias.rs/issues>&gt;

## SEE ALSO

&lt;<https://openalias.org>&gt;<br />
&lt;<https://github.com/nabijaczleweli/openalias.rs>&gt;
