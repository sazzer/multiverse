import { Button, Input } from "../components/form";
import React, { useEffect, useState } from "react";

import { Spinner } from "../components/spinner";
import { useForm } from "react-hook-form";

export const ProfileView: React.FC = () => {
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    setTimeout(() => {
      setLoading(false);
    }, 3000);
  }, []);
  const { register, handleSubmit, errors } = useForm({
    defaultValues: {
      username: "sazzer",
      email_address: "graham@grahamcox.co.uk",
      display_name: "Graham",
    },
  });

  const onSubmitHandler = console.log;

  if (loading) {
    return <Spinner />;
  }

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <fieldset>
        <Input
          id="username"
          i18n="profile.profile.username"
          required
          readOnly
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="email_address"
          i18n="profile.profile.email_address"
          error={errors.email_address}
          type="email"
          required
          autoFocus
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="display_name"
          i18n="profile.profile.display_name"
          error={errors.display_name}
          required
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />

        <div className="btn-group form-group">
          <Button label="profile.profile.submit" type="submit" />
          <Button
            label="profile.profile.cancel"
            type="reset"
            display="secondary"
          />
        </div>
      </fieldset>
    </form>
  );
};
